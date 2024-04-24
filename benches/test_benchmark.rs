use criterion::{black_box, criterion_group, criterion_main, Criterion};
use genbank_parser::{
    faster::parse_new_features, faster::parse_new_sequence_record, parse_features,
    parse_sequence_record, parse_sequence_record_by_positions,
};

fn test_parse_sequence_record_by_positions(record: &[u8]) {
    _ = parse_sequence_record_by_positions(record);
}

fn test_parse_sequence_record(record: &[u8]) {
    (_, _) = parse_sequence_record(record);
}

fn test_parse_new_sequence_record(record: &[u8]) {
    (_, _) = parse_new_sequence_record(record);
}

fn test_parse_features(data: &[u8]) {
    let feature_lines = data.split(|&b| b == b'\n');
    _ = parse_features(feature_lines);
}

fn test_parse_new_features(data: &[u8]) {
    let feature_lines = data.split(|&b| b == b'\n');
    _ = parse_new_features(feature_lines);
}

fn alignment_benchmark(c: &mut Criterion) {
    let features: &[u8] = black_box(
        b"
     source          1..674
                     /organism=\"Influenza A virus (A/Sendai/TU65/2006(H1N1))\"
                     /mol_type=\"viral cRNA\"
                     /strain=\"A/Sendai/TU65/2008\"
                     /serotype=\"H1N1\"
                     /isolate=\"laboratory strain 4\"
                     /host=\"Homo sapiens\"
                     /db_xref=\"taxon:672336\"
                     /segment=\"7\"
                     /lab_host=\"Canis lupus familiaris MDCK cells\"
                     /country=\"Japan: Miyagi, Sendai\"
                     /note=\"This virus is laboratory strain. The virus was
                     cultured in MDCK cells in the presence of amantadine after
                     plaque-purification.\"
     gene            <1..440
                     /gene=\"M1\"
     CDS             <1..440
                     /gene=\"M1\"
                     /codon_start=3
                     /product=\"matrix protein 1\"
                     /protein_id=\"BAI79497.1\"
                     /translation=\"TFHGAKEIALSYSAGALASCMGLIYNRMGAVTTESAFGLICATC
                     EQIADSQHKSHRQMVTTTNPLIRHENRMVLASTTAKAMEQMAGSSEQAAEAMEVASQA
                     RQMVQAMRAIGTHPSSSTGLKNDLLENLQAYQKRMGVQMQRFK\"
     gene            <396..663
                     /gene=\"M2\"
     CDS             <396..663
                     /gene=\"M2\"
                     /codon_start=2
                     /product=\"matrix protein 2\"
                     /protein_id=\"BAI79498.1\"
                     /translation=\"PIRNEWGCRCNDSSDPLVVAASIIEIVHLILWIIDRLFSKSICR
                     IFKHGLKRGPSTEGIPESMREEYREEQRNAVDADDDHFVSIELE\"",
    );
    let genbank_record = black_box(
        b"LOCUS       AB000048                2007 bp    DNA     linear   VRL 14-JUL-2009
DEFINITION  Feline panleukopenia virus gene for nonstructural protein 1,
            complete cds, isolate: 483.
ACCESSION   AB000048
VERSION     AB000048.1
KEYWORDS    .
SOURCE      Feline panleukopenia virus
  ORGANISM  Feline panleukopenia virus
            Viruses; Monodnaviria; Shotokuvirae; Cossaviricota;
            Quintoviricetes; Piccovirales; Parvoviridae; Parvovirinae;
            Protoparvovirus; Protoparvovirus carnivoran1.
REFERENCE   1
  AUTHORS   Horiuchi,M.
  TITLE     Evolutionary pattern of feline panleukopenia virus differs from
            that of canine parvovirus
  JOURNAL   Unpublished
REFERENCE   2  (bases 1 to 2007)
  AUTHORS   Horiuchi,M.
  TITLE     Direct Submission
  JOURNAL   Submitted (22-DEC-1996) Contact:Motohiro Horiuchi Obihiro
            University of Agriculture and Veterinary Medicine, Veterinary
            Public Health; Inada cho, Obihiro, Hokkaido 080, Japan
FEATURES             Location/Qualifiers
     source          1..2007
                     /organism=\"Feline panleukopenia virus\"
                     /mol_type=\"genomic DNA\"
                     /isolate=\"483\"
                     /db_xref=\"taxon:10786\"
                     /lab_host=\"Felis domesticus\"
     CDS             1..2007
                     /codon_start=1
                     /product=\"nonstructural protein 1\"
                     /protein_id=\"BAA19009.1\"
                     /translation=\"MSGNQYTEEVMEGVNWLKKHAEDEAFSFVFKCDNVQLNGKDVRW
                     NNYTKPIQNEELTSLIRGAQTAMDQTEEEEMDWESEVDSLAKKQVQTFDALIKKCLFE
                     VFVSKNIEPNECVWFIQHEWGKDQGWHCHVLLHSKNLQQATGKWLRRQMNMYWSRWLV
                     TLCSINLTPTEKIKLREIAEDSEWVTILTYRHKQTKKDYVKMVHFGNMIAYYFLTKKK
                     IVHMTKESGYFLSTDSGWKFNFMKYQDRHTVSTLYTEQMKPETVETTVTTAQETKRGR
                     IQTKKEVSIKCTLRDLVSKRVTSPEDWMMLQPDSYIEMMAQPGGENLLKNTLEICTLT
                     LARTKTAFELILEKADNTKLTNFDLANSRTCQIFRMHGWNWIKVCHAIACVLNRQGGK
                     RNTVLFHGPASTGKSIIAQAIAQAVGNVGCYNAANVNFPFNDCTNKNLIWVEEAGNFG
                     QQVNQFKAICSGQTIRIDQKGKGSKQIEPTPVIMTTNENITIVRIGCEERPEHTQPIR
                     DRMLNIKLVCKLPGDFGLVDKEEWPLICAWLVKHGYQSTMANYTHHWGKVPEWDENWA
                     EPKIQEGINSPGCKDLETQAASNPQSQDHVLTPLTPDVVDLALEPWSTPDTPIAETAN
                     QQSNQLGVTHKDVQASPTWSEIEADLRAIFTSEQLEEDFRDDLD\"
ORIGIN      
        1 atgtctggca accagtatac tgaggaagtt atggagggag taaattggtt aaagaaacat
       61 gcagaagatg aagcattttc gtttgttttt aaatgtgaca acgtccaact aaatggaaag
      121 gatgttcgct ggaacaacta taccaaacca attcaaaatg aagagctaac atctttaatt
      181 agaggagcac aaacagcaat ggatcaaacc gaagaagaag aaatggactg ggaatcggaa
      241 gttgatagtc tcgccaaaaa gcaagtacaa acttttgatg cattaattaa aaaatgtctt
      301 tttgaagtct ttgtttctaa aaatatagaa ccaaatgaat gtgtttggtt tattcaacat
      361 gaatggggaa aagatcaagg ctggcattgt catgttttac ttcatagtaa gaacttacaa
      421 caagcaactg gtaaatggct acgcagacaa atgaatatgt attggagtag atggttggtg
      481 actctttgtt cgataaattt aacaccaact gaaaagatta agctcagaga aattgcagaa
      541 gatagtgaat gggtaactat attaacatac agacataagc aaacaaaaaa agactatgtt
      601 aaaatggttc attttggaaa tatgatagca tattactttt taacaaagaa aaaaattgtc
      661 cacatgacaa aagaaagtgg ctatttttta agtactgatt ctggttggaa atttaacttt
      721 atgaagtatc aagacagaca tactgtcagc acactttaca ctgaacaaat gaaaccagaa
      781 accgttgaaa ccacagtgac gacagcacag gaaacaaagc gcgggagaat tcaaactaaa
      841 aaggaagtgt caatcaaatg tactttgcgg gacttggtta gtaaaagagt aacatcacct
      901 gaagactgga tgatgttaca accagatagt tatattgaaa tgatggcaca accaggaggt
      961 gaaaatctct taaaaaatac acttgaaatt tgtactttga ctttagcaag aacaaaaaca
     1021 gcatttgaat taatacttga aaaagcagat aataccaaac taactaactt tgatcttgca
     1081 aattctagaa catgtcaaat ttttagaatg cacggatgga attggattaa agtttgtcac
     1141 gctatagcat gtgttttaaa tagacaaggt ggtaaaagaa atacagttct ttttcatgga
     1201 ccagcaagta caggaaaatc tattattgct caagccatag cacaagctgt gggtaatgtt
     1261 ggttgctata atgcagcaaa tgtaaatttt ccatttaatg actgtaccaa taaaaattta
     1321 atttgggttg aagaagctgg taactttggt caacaagtta atcaatttaa agcaatttgt
     1381 tctggacaaa caattagaat tgatcaaaaa ggtaaaggaa gtaagcaaat tgaaccaact
     1441 ccagtaatta tgacaactaa tgaaaatata acaattgtaa gaattggatg tgaagaaaga
     1501 cctgaacata cacaaccaat aagagacaga atgttgaaca ttaaattagt atgtaagctt
     1561 ccaggagact ttggtttggt tgataaagaa gaatggcctt taatatgtgc atggttagtt
     1621 aaacatggtt atcaatcaac catggctaac tacacacatc attggggaaa agtaccagag
     1681 tgggatgaaa actgggcgga gcctaaaata caagaaggta taaattcacc aggttgcaaa
     1741 gacttagaga cacaagcggc aagcaatcct cagagtcaag accacgttct aactcctctg
     1801 actccggacg tagtggacct tgcactggaa ccgtggagta ctccagatac gcctattgca
     1861 gaaactgcaa atcaacaatc aaaccaactt ggcgttactc acaaagacgt gcaagcgagt
     1921 ccgacatggt ccgaaataga ggcagacctg agagccattt ttacttctga acaattggaa
     1981 gaagattttc gagacgactt ggattaa
//",
    );
    c.bench_function("test parse_sequence_record_by_positions", |b| {
        b.iter(|| test_parse_sequence_record_by_positions(genbank_record))
    });
    c.bench_function("test parse_sequence_record", |b| {
        b.iter(|| test_parse_sequence_record(genbank_record))
    });
    c.bench_function("test parse_new_sequence_record", |b| {
        b.iter(|| test_parse_new_sequence_record(genbank_record))
    });
    c.bench_function("test parse_features", |b| {
        b.iter(|| test_parse_features(features))
    });
    c.bench_function("test parse_new_features", |b| {
        b.iter(|| test_parse_new_features(features))
    });
}

criterion_group!(benchs, alignment_benchmark);
criterion_main!(benchs);
