use serde::{ser::Error, Serialize, Serializer};
use std::fmt;

#[inline]
fn serialize_as_utf8<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string = std::str::from_utf8(bytes).map_err(S::Error::custom)?;
    serializer.serialize_str(string)
}

#[derive(Debug, Serialize)]
pub struct Sequence {
    #[serde(serialize_with = "serialize_as_utf8")]
    pub version: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub definition: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub organism: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub taxonomy: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub sequence: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub host: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub mol_type: Vec<u8>,
}

impl Sequence {
    fn append_data(&mut self, data_type: &DataType, data: &[u8]) {
        match data_type {
            DataType::Definition => self.definition.extend_from_slice(data),
            DataType::Version => self.version.extend_from_slice(data),
            DataType::Taxonomy => self.taxonomy.extend_from_slice(data),
            DataType::Organism => self.organism.extend_from_slice(data),
            _ => (),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Protein {
    #[serde(serialize_with = "serialize_as_utf8")]
    pub protein_id: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub source_id: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub sequence: Vec<u8>,
    #[serde(serialize_with = "serialize_as_utf8")]
    pub location: Vec<u8>,
}

impl Protein {
    fn new() -> Self {
        Protein {
            protein_id: Vec::new(),
            source_id: Vec::new(),
            sequence: Vec::new(),
            location: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Feature {
    feature_type: Option<FeatureType>,
    location: Vec<u8>,
    qualifiers: Vec<(Vec<u8>, Vec<u8>)>,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let feature_type_str = match &self.feature_type {
            Some(feature_type) => feature_type.to_string(),
            None => String::from("None"),
        };
        let location_str = String::from_utf8_lossy(&self.location);
        let qualifiers_str = self
            .qualifiers
            .iter()
            .map(|(name, value)| {
                format!(
                    "{}: {}",
                    String::from_utf8_lossy(name),
                    String::from_utf8_lossy(value)
                )
            })
            .collect::<Vec<String>>()
            .join(", ");

        write!(
            f,
            "{}\tLocation: {}\tQualifiers: [{}]",
            feature_type_str, location_str, qualifiers_str
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum FeatureType {
    Source,
    Gene,
    CDS,
    Other,
}

impl FeatureType {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match trim_ascii(bytes) {
            b"" => None,
            b"source" => Some(Self::Source),
            b"gene" => Some(Self::Gene),
            b"CDS" => Some(Self::CDS),
            _ => Some(Self::Other),
        }
    }
}

impl fmt::Display for FeatureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FeatureType::Source => write!(f, "source"),
            FeatureType::Gene => write!(f, "gene"),
            FeatureType::CDS => write!(f, "CDS"),
            FeatureType::Other => write!(f, "other"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum DataType {
    Definition,
    Version,
    Organism,
    Origin,
    Source,
    Taxonomy,
    Features,
    Other,
}

impl DataType {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match trim_ascii(bytes) {
            b"" => None,
            b"DEFINITION" => Some(Self::Definition),
            b"VERSION" => Some(Self::Version),
            b"SOURCE" => Some(Self::Source),
            b"ORGANISM" => Some(Self::Organism),
            b"ORIGIN" => Some(Self::Origin),
            b"FEATURES" => Some(Self::Features),
            _ => Some(Self::Other),
        }
    }

    fn is_data_complete(&self) -> bool {
        matches!(self, DataType::Origin | DataType::Other) // Feature?
    }
}

#[inline]
fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|&b| !b.is_ascii_whitespace())
        .unwrap_or(0);
    let end = bytes
        .iter()
        .rposition(|&b| !b.is_ascii_whitespace())
        .map_or(0, |pos| pos + 1);
    &bytes[start..end]
}

pub fn split_on_delimiter<'a>(
    data: &'a [u8],
    separator: &[u8],
    include_separator: bool,
) -> Vec<&'a [u8]> {
    // todo: specify delimeter at beg or end when include_separator=True??
    // note: this function might be super slow, vs having a separate function for each include_separator value
    let mut parts = Vec::new();
    let mut start = 0;

    for (i, window) in data.windows(separator.len()).enumerate() {
        if window == separator && i > 0 {
            // is this i > 0 okay? could enumerate().skip(separator.len() if include_separator)?
            parts.push(&data[start..i]);
            start = if include_separator {
                i
            } else {
                i + separator.len()
            };
        }
    }

    if start < data.len() {
        parts.push(&data[start..]);
    }

    parts
}

#[inline]
fn remove_quotes(data: &[u8]) -> &[u8] {
    if data.starts_with(&[b'"']) && data.ends_with(&[b'"']) && data.len() >= 2 {
        &data[1..data.len() - 1]
    } else {
        data
    }
}

fn parse_features<'a, I>(data: I) -> Result<Vec<Feature>, &'static str>
where
    I: Iterator<Item = &'a [u8]>,
{
    const FEATURE_QUALIFIER_INDENT: usize = 21;
    let feature_qualifier_spacer = [b' '; FEATURE_QUALIFIER_INDENT];
    // should be same num spaces as FEATURE_QUALIFIER_INDENT
    let feature_qualifier_delimeter = b"                     /";
    // println!("Data: {}", String::from_utf8_lossy(&data));

    let feature_data =
        data.filter(|&line| !line.is_empty())
            .fold(Vec::new(), |mut acc: Vec<Vec<u8>>, line| {
                if line.starts_with(&feature_qualifier_spacer) {
                    if let Some(last) = acc.last_mut() {
                        if line[FEATURE_QUALIFIER_INDENT] == b'/' {
                            last.extend_from_slice(line);
                        } else {
                            // note: this might not be safe for all multiline qualifiers, but works for translations
                            last.extend_from_slice(trim_ascii(line));
                        }
                    } else {
                        acc.push(line.to_vec());
                    }
                } else {
                    acc.push(line.to_vec());
                }
                acc
            });

    let mut features = Vec::new();

    for feature_line in feature_data {
        // let lines = feature_line.split(|&b| b == b'/');
        let lines = split_on_delimiter(&feature_line, feature_qualifier_delimeter, false);

        let mut feature = Feature {
            feature_type: None,
            location: Vec::new(),
            qualifiers: Vec::new(),
        };
        // qualifiers
        for (i, line) in lines.iter().enumerate() {
            // println!("Line ({}): {}", i, String::from_utf8_lossy(&line));
            if i == 0 {
                if line.len() < FEATURE_QUALIFIER_INDENT {
                    return Err("Line is too short for feature.");
                }
                feature.feature_type = FeatureType::from_bytes(&line[..FEATURE_QUALIFIER_INDENT]);
                feature
                    .location
                    .extend_from_slice(trim_ascii(&line[FEATURE_QUALIFIER_INDENT..]));
            } else {
                let qualifier: Vec<&[u8]> = line.split(|&b| b == b'=').collect();
                if qualifier.len() == 2 {
                    let (qualifier_name, qualifier_value) =
                        (qualifier[0], remove_quotes(qualifier[1]));
                    feature
                        .qualifiers
                        .push((qualifier_name.to_vec(), qualifier_value.to_vec()));
                } else {
                    feature
                        .qualifiers
                        .push((qualifier[0].to_vec(), b"".to_vec()));
                }
            }
        }
        // println!("Feature: {}", feature);
        features.push(feature);
    }

    Ok(features)
}

pub fn parse_sequence_record(record: &[u8]) -> (Sequence, Vec<Protein>) {
    const GENBANK_INDENT: usize = 12;
    // let genbank_spacer = [b' '; GENBANK_INDENT];

    let mut sequence = Sequence {
        version: Vec::new(),
        definition: Vec::new(),
        taxonomy: Vec::new(),
        organism: Vec::new(),
        sequence: Vec::new(),
        host: Vec::new(),
        mol_type: Vec::new(),
    };
    let mut origin = Vec::new();
    let mut features = Vec::new();

    let mut lines = record.split(|&b| b == b'\n');

    let mut data_type: DataType = DataType::Other;
    for line in &mut lines {
        if line.len() < GENBANK_INDENT {
            panic!(
                "Line is too short for sequence: {:?}",
                String::from_utf8(sequence.version.clone())
            );
        }

        let line_type = DataType::from_bytes(&line[..GENBANK_INDENT]);
        // println!("line_type: {:?}", line_type);
        match line_type {
            None => {
                if !data_type.is_data_complete() {
                    sequence.append_data(&data_type, &[b' ']);
                }
            }
            Some(DataType::Features) => {
                // features followed by origin
                let feature_lines = lines
                    .by_ref()
                    .take_while(|slice| !slice.starts_with(b"ORIGIN")); // ORIGIN line consumed here?
                features = parse_features(feature_lines).expect(&format!(
                    "Failed to parse features for sequence: {:?}",
                    String::from_utf8(sequence.version.clone())
                ));

                origin = lines
                    .flat_map(|slice| slice)
                    .filter(|&&b| b.is_ascii_alphabetic())
                    .copied()
                    .collect();
                break;
            }
            Some(d_type) => {
                data_type = d_type;
            }
        }

        if data_type == DataType::Organism && line.contains(&b';') {
            data_type = DataType::Taxonomy;
        }

        if !data_type.is_data_complete() {
            sequence.append_data(&data_type, &line[GENBANK_INDENT..]);
        }
    }

    // todo: handle features
    let mut proteins = Vec::new();
    for feature in features {
        match feature.feature_type {
            Some(FeatureType::Source) => {
                for (qualifier_name, qualifier_value) in feature.qualifiers {
                    match qualifier_name.as_slice() {
                        b"host" | b"lab_host" => {
                            sequence.host = qualifier_value;
                        }
                        b"mol_type" => {
                            sequence.mol_type = qualifier_value;
                        }
                        _ => (),
                    }
                }
            }
            Some(FeatureType::CDS) => {
                let mut protein = Protein::new();
                for (qualifier_name, qualifier_value) in feature.qualifiers {
                    match qualifier_name.as_slice() {
                        b"protein_id" => {
                            protein.protein_id = qualifier_value;
                        }
                        b"translation" => {
                            protein.sequence = qualifier_value;
                        }
                        _ => (),
                    }
                }
                if !protein.protein_id.is_empty() && !protein.sequence.is_empty() {
                    protein.location = feature.location;
                    protein.source_id = sequence.version.clone();
                    proteins.push(protein);
                }
            }
            _ => (),
        }
    }
    sequence.sequence = origin;
    // println!("Num proteins: {}", proteins.len());

    (sequence, proteins)
}

pub fn parse_sequence_record_by_positions(record: &[u8]) -> Sequence {
    let definition_start = record
        .windows(11)
        .position(|window| window == b"DEFINITION ")
        .map(|pos| pos + 11)
        .unwrap_or(record.len());

    let definition_end = record
        .windows(10)
        .skip(definition_start)
        .position(|window| window == b"ACCESSION ")
        .unwrap_or(record.len());

    let (defintion, record) = record[definition_start..].split_at(definition_end);

    let defintion: Vec<u8> = defintion
        .split(|&b| b.is_ascii_whitespace())
        .filter(|&slice| !slice.is_empty())
        .flat_map(|slice| slice.iter().chain(std::iter::once(&b' ')))
        .copied()
        .collect();

    // unsafe??
    let definition = defintion[..defintion.len() - 1].to_vec();

    let version_start = record
        .windows(8)
        .position(|window| window == b"VERSION ")
        .map(|pos| pos + 8)
        .unwrap_or(0);

    let version_end = record[version_start..]
        .iter()
        .position(|&b| b == b'\n')
        .unwrap_or(record.len());

    let (version, record) = record[version_start..].split_at(version_end);

    let version = version
        .iter()
        .filter(|&&b| !b.is_ascii_whitespace())
        .copied()
        .collect();

    let taxonomy_start = record
        .windows(9)
        .position(|window| window == b"ORGANISM ")
        .map(|pos| pos + 10)
        .unwrap_or(record.len());

    let taxonomy_end = record
        .windows(10)
        .skip(taxonomy_start)
        .position(|window| window.starts_with(b"REFERENCE "))
        .unwrap_or(record.len());

    let (taxonomy, record) = record[taxonomy_start..].split_at(taxonomy_end);

    let taxonomy: Vec<u8> = taxonomy
        .iter()
        .skip_while(|&&b| b != b'\n')
        .skip(1)
        .copied()
        .collect::<Vec<_>>()
        .split(|&b| b.is_ascii_whitespace())
        .filter(|&slice| !slice.is_empty())
        .flat_map(|slice| slice.iter().chain(std::iter::once(&b' ')))
        .copied()
        .collect();

    let taxonomy = taxonomy[..taxonomy.len() - 1].to_vec();

    let origin_start = record
        .windows(7)
        .position(|window| window == b"ORIGIN ")
        .map(|pos| pos + 7)
        .unwrap_or(record.len());

    let sequence = record[origin_start..]
        .iter()
        .filter(|&&b| b.is_ascii_alphabetic())
        .copied()
        .collect();

    let organism = Vec::from([1]);
    let host = Vec::from([1]);
    let mol_type = Vec::from([1]);

    Sequence {
        version,
        definition,
        organism,
        taxonomy,
        sequence,
        host,
        mol_type,
    }
}
