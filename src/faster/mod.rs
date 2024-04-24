use crate::{remove_quotes, split_at_sequence, trim_ascii, FeatureType, SequnceDataType};

#[derive(Debug)]
pub struct NewSequence<'a> {
    pub version: Option<&'a [u8]>,
    pub definition: Vec<&'a [u8]>,
    organism: Option<&'a [u8]>,
    pub taxonomy: Vec<&'a [u8]>,

    pub sequence: Option<Vec<&'a u8>>,
    host: Option<Vec<&'a [u8]>>,
    mol_type: Option<Vec<&'a [u8]>>,
    // host: Option<Vec<&'a u8>>,
    // mol_type: Option<Vec<&'a u8>>,
}

impl<'a> NewSequence<'a> {
    fn new() -> NewSequence<'a> {
        NewSequence {
            version: None,
            definition: Vec::new(),
            organism: None,
            taxonomy: Vec::new(),
            sequence: None,
            host: None,
            mol_type: None,
        }
    }

    fn append_data(&mut self, data_type: &SequnceDataType, data: &'a [u8]) {
        match data_type {
            SequnceDataType::Definition => self.definition.push(data),
            SequnceDataType::Version => self.version = Some(data),
            SequnceDataType::Taxonomy => self.taxonomy.push(data),
            SequnceDataType::Organism => self.organism = Some(data),
            _ => (),
        }
    }
}

// impl<'a> Serialize for NewSequence<'a> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("NewSequence", 7)?;

//         let definition: Vec<u8> = self.definition.iter().flat_map(|&s| s.iter().copied()).collect();
//         let taxonomy: Vec<u8> = self.taxonomy.iter().flat_map(|&s| s.iter().copied()).collect();
//         let host: Vec<u8> = self.host.as_ref().unwrap_or(&Vec::new()).iter().flat_map(|&s| s.iter().copied()).collect();
//         // let host: Vec<u8> = self.host.iter().flat_map(|v| v.iter().flat_map(|&s| s.iter().copied())).collect();
//         // let mol_type: Vec<u8> = self.mol_type.iter().flat_map(|v| v.iter().flat_map(|&s| s.iter().copied())).collect();
//         let mol_type: Vec<u8> = self.mol_type.as_ref().unwrap().iter().flat_map(|&s| s.iter().copied()).collect();

//         state.serialize_field("version", &self.version.map(|s: &[u8]| s.to_vec()))?;
//         state.serialize_field("definition", &definition)?;
//         state.serialize_field("taxonomy", &taxonomy)?;
//         state.serialize_field("host", &host)?;
//         state.serialize_field("mol_type", &mol_type)?;
//         state.serialize_field("organism", &self.organism.map(|s| s.to_vec()))?;
//         state.serialize_field("sequence", &self.sequence.as_ref().map(|v| v.iter().map(|&s| *s).collect::<Vec<u8>>()))?;

//         state.end()
//     }
// }

#[derive(Debug)]
pub struct NewFeature<'a> {
    pub feature_type: Option<FeatureType>,
    pub location: Option<&'a [u8]>,
    // qualifiers: Vec<(&'a [u8], Vec<&'a u8>)>,
    pub qualifiers: Vec<(&'a [u8], Vec<&'a [u8]>)>,
}

#[derive(Debug)]
pub struct NewProtein<'a> {
    // pub protein_id: Option<Vec<&'a u8>>,
    pub protein_id: Option<Vec<&'a [u8]>>,
    pub source_id: Option<&'a [u8]>,
    // pub sequence: Option<Vec<&'a u8>>,
    pub sequence: Option<Vec<&'a [u8]>>,
    pub location: Option<&'a [u8]>,
}

impl<'a> NewProtein<'a> {
    fn new() -> Self {
        NewProtein {
            protein_id: None,
            source_id: None,
            sequence: None,
            location: None,
        }
    }
}

// impl<'a> Serialize for NewProtein<'a> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::ser::Serializer,
//     {
//         let mut state = serializer.serialize_struct("NewProtein", 4)?;

//         let protein_id: Vec<u8> = self.protein_id.as_ref().unwrap().iter().flat_map(|&s| s.iter().copied()).collect();
//         let sequence: Vec<u8> = self.sequence.as_ref().unwrap().iter().flat_map(|&s| s.iter().copied()).collect();

//         state.serialize_field("protein_id", &protein_id)?;
//         state.serialize_field("source_id", &self.source_id.map(|s| s.to_vec()))?;
//         state.serialize_field("sequence", &sequence)?;
//         state.serialize_field("location", &self.location.map(|s| s.to_vec()))?;

//         state.end()
//     }
// }

pub fn parse_new_features<'a, I>(data: I) -> Result<Vec<NewFeature<'a>>, &'static str>
where
    I: Iterator<Item = &'a [u8]>,
{
    const FEATURE_QUALIFIER_INDENT: usize = 21;
    let feature_qualifier_spacer = [b' '; FEATURE_QUALIFIER_INDENT];
    // println!("Data: {}", String::from_utf8_lossy(&data));

    let mut features = Vec::new();
    let mut feature: Option<NewFeature> = None;
    let mut qualifier_name: Option<&[u8]> = None;
    // let mut qualifier_value: Option<Vec<&u8>> = None;
    let mut qualifier_value: Option<Vec<&[u8]>> = None;

    // data is an iterator split on new lines
    let lines = data.filter(|&line| !line.is_empty());
    for line in lines {
        // println!("Line: {}", String::from_utf8_lossy(&line));
        if line.starts_with(&feature_qualifier_spacer) {
            // println!("Qualifier line: {}", String::from_utf8_lossy(&line));
            let line_slice = &line[FEATURE_QUALIFIER_INDENT..];
            // println!("Qualifier line slice: {}", String::from_utf8_lossy(&line_slice));
            if line_slice[0] == b'/' {
                // if line_slice.starts_with(b"/") {
                // does this consume the first value?
                if let Some(filled_qualifier_name) = qualifier_name {
                    if let Some(ref mut filled_feature) = feature {
                        filled_feature
                            .qualifiers
                            .push((filled_qualifier_name, qualifier_value.unwrap_or(Vec::new())));
                    }
                }
                let qualifier: Vec<&[u8]> = line_slice.split(|&b| b == b'=').collect();
                // println!("Qualifier len: {}", qualifier.len());
                if qualifier.len() == 2 {
                    let (left, right) = (qualifier[0], remove_quotes(qualifier[1])); // should qualifier[1] be ascii trimmed too?
                                                                                     // println!("Qualifier line left: {}, right: {}", String::from_utf8_lossy(&left), String::from_utf8_lossy(&right));
                    qualifier_name = Some(&left[1..]);
                    // qualifier_value = Some(right.iter().collect());
                    qualifier_value = Some(vec![right]);
                } else {
                    qualifier_name = Some(&qualifier[0][1..]);
                    qualifier_value = Some(Vec::new());
                }
            } else if let Some(ref mut filled_qualifier_value) = qualifier_value {
                // filled_qualifier_value.extend(remove_quotes(trim_ascii(line_slice)));
                filled_qualifier_value.push(remove_quotes(trim_ascii(line_slice)));
            }
        } else {
            // if there is a qualifier name and value filled in, take them and add to existing feature
            if let Some(filled_qualifier_name) = qualifier_name.take() {
                if let Some(ref mut filled_feature) = feature {
                    filled_feature.qualifiers.push((
                        filled_qualifier_name,
                        qualifier_value.take().unwrap_or(Vec::new()),
                    ));
                }
            }
            if let Some(filled_feature) = feature.take() {
                // println!("(1) Pushing feature: {:?} with attributes:", &filled_feature.feature_type);
                // for qualifier in &filled_feature.qualifiers {
                //     let (q_name, q_value) = qualifier;
                //     println!("\tq_name: {:?}, q_value: {:?}", String::from_utf8(q_name.to_vec()), String::from_utf8(q_value.iter().copied().flat_map(|slice| slice).copied().collect()));
                // }
                features.push(filled_feature);
            }
            feature = Some(NewFeature {
                feature_type: FeatureType::from_bytes(&line[..FEATURE_QUALIFIER_INDENT]),
                location: Some(trim_ascii(&line[FEATURE_QUALIFIER_INDENT..])),
                qualifiers: Vec::new(),
            });
        }
    }
    // println!("final qualifier name: {:?}", qualifier_name);
    if let Some(filled_qualifier_name) = qualifier_name {
        if let Some(filled_feature) = feature.as_mut() {
            filled_feature
                .qualifiers
                .push((filled_qualifier_name, qualifier_value.unwrap()));
        }
    }
    // println!("final feature: {:?}", feature);
    if let Some(filled_feature) = feature {
        features.push(filled_feature);
    }
    // for feature in &features {
    //     println!("feature type: {:?}", feature.feature_type);
    // }
    Ok(features)
}

pub fn parse_new_sequence_record<'a>(record: &'a [u8]) -> (NewSequence<'a>, Vec<NewProtein<'a>>) {
    const GENBANK_INDENT: usize = 12;
    // let genbank_spacer = [b' '; GENBANK_INDENT];
    let mut sequence = NewSequence::new();

    let (lines, ending_lines) = split_at_sequence(record, b"\nFEATURES").unwrap();
    // todo: panic here if needed
    let (feature_lines, origin_lines) = split_at_sequence(ending_lines, b"\nORIGIN").unwrap();
    let features = parse_new_features(feature_lines.split(|&b| b == b'\n'))
        .expect("Failed to parse features for sequence");
    // println!("Features: {:?}", features);
    // let features = parse_new_features(feature_lines.split(|&b| b == b'\n')).expect(&format!(
    //     "Failed to parse features for sequence: {:?}",
    //     String::from_utf8(sequence.version.unwrap().to_vec())
    // ));
    sequence.sequence = Some(
        origin_lines
            .iter()
            .filter(|&&b| b.is_ascii_alphabetic())
            .collect(),
    );
    // sequence.sequence = Some(origin_lines); // Some(origin_lines.iter().cloned().filter(|&b| b.is_ascii_alphabetic()).collect::<Vec<u8>>().as_slice());

    let mut lines = lines.split(|&b| b == b'\n');
    let mut data_type: SequnceDataType = SequnceDataType::Other;
    for line in &mut lines {
        if line.len() < GENBANK_INDENT {
            panic!(
                "Line is too short for sequence: {:?}\nline: {:?}",
                String::from_utf8(sequence.version.unwrap().to_vec()),
                String::from_utf8(line.to_vec()),
            );
        }

        let line_type = SequnceDataType::from_bytes(&line[..GENBANK_INDENT]);
        // println!("line_type: {:?}", line_type);
        match line_type {
            None => {
                if !data_type.is_data_complete() {
                    sequence.append_data(&data_type, &[b' ']);
                }
            }
            Some(d_type) => {
                data_type = d_type;
            }
        }

        if data_type == SequnceDataType::Organism && line.contains(&b';') {
            data_type = SequnceDataType::Taxonomy;
        }

        if !data_type.is_data_complete() {
            sequence.append_data(&data_type, &line[GENBANK_INDENT..]);
        }
    }

    // for feature in &features {
    //     println!("feature type: {:?}", feature.feature_type);
    // }
    let mut proteins = Vec::new();
    for feature in features {
        match feature.feature_type {
            Some(FeatureType::Source) => {
                for (qualifier_name, mut qualifier_value) in feature.qualifiers {
                    match qualifier_name {
                        b"host" | b"lab_host" => {
                            sequence.host = Some(std::mem::take(&mut qualifier_value));
                        }
                        b"mol_type" => {
                            sequence.mol_type = Some(std::mem::take(&mut qualifier_value));
                        }
                        _ => (),
                    }
                }
            }
            Some(FeatureType::CDS) => {
                // println!("CDS feature found!");
                // for qualifier in &feature.qualifiers {
                //     let (q_name, q_value) = qualifier;
                //     println!("q_name: {:?}, q_value: {:?}", String::from_utf8(q_name.to_vec()), q_value);
                // }
                let mut protein = NewProtein::new();
                for (qualifier_name, mut qualifier_value) in feature.qualifiers {
                    // println!("qualifier_name: {:?}", String::from_utf8(qualifier_name.to_vec()));
                    match qualifier_name {
                        b"protein_id" => {
                            // println!("protein_id: {:?}", String::from_utf8(qualifier_value.iter().flat_map(|&slice| slice).copied().collect()));
                            protein.protein_id = Some(std::mem::take(&mut qualifier_value));
                        }
                        b"translation" => {
                            protein.sequence = Some(std::mem::take(&mut qualifier_value));
                        }
                        _ => (),
                    }
                }
                if !protein.protein_id.is_none() && !protein.sequence.is_none() {
                    protein.location = feature.location;
                    protein.source_id = sequence.version;
                    proteins.push(protein);
                }
            }
            _ => (),
        }
    }
    // println!("Num proteins: {}", proteins.len());

    (sequence, proteins)
}
