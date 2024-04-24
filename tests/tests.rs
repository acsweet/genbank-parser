use genbank_parser::{
    faster::parse_new_sequence_record, parse_sequence_record, parse_sequence_record_by_positions,
    split_on_delimiter,
};

const RECORD: &[u8] =
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
//";

const RECORD_2: &[u8] =
    b"LOCUS       AF148865                  81 bp    RNA     linear   VRL 26-JUL-2016
DEFINITION  Norwalk-like virus strain Gat010-02/97-QC RNA polymerase gene,
            partial cds.
ACCESSION   AF148865
VERSION     AF148865.1
KEYWORDS    .
SOURCE      Norwalk-like virus
  ORGANISM  Norwalk-like virus
            Viruses; Riboviria; Orthornavirae; Pisuviricota; Pisoniviricetes;
            Picornavirales; Caliciviridae; Norovirus; Norwalk virus.
REFERENCE   1  (bases 1 to 81)
  AUTHORS   Gonin,P., Couillard,M. and d'Halewyn,M.A.
  TITLE     Genetic diversity and molecular epidemiology of Norwalk-like
            viruses
  JOURNAL   J. Infect. Dis. 182 (3), 691-697 (2000)
   PUBMED   10950761
REFERENCE   2  (bases 1 to 81)
  AUTHORS   Gonin,P. and Couillard,M.
  TITLE     Direct Submission
  JOURNAL   Submitted (06-MAY-1999) Biologie Moleculaire-Virologie, Laboratoire
            de Sante Publique du Quebec, 20045 Chemin Sainte-Marie,
            Sainte-Anne-de-Bellevue, PQ H9X 3R5, Canada
FEATURES             Location/Qualifiers
     source          1..81
                     /organism=\"Norwalk-like virus\"
                     /mol_type=\"genomic RNA\"
                     /strain=\"Gat010-02/97-QC\"
                     /host=\"Homo sapiens\"
                     /db_xref=\"taxon:95340\"
     CDS             <1..>81
                     /codon_start=1
                     /product=\"RNA polymerase\"
                     /protein_id=\"AAF73744.1\"
                     /translation=\"LLTLCALSEVTNLSPDIIQANSLFSFY\"
ORIGIN      
        1 cttctcactc tctgtgcgct ctctgaagtt acaaacttgt cccctgacat aatacaggct
       61 aattccctct tctcctttta t";

const RECORD_3: &[u8] =
    b"LOCUS       AF219750                 754 bp    DNA     linear   VRL 02-JAN-2001
DEFINITION  HIV-1 LTS 38d from Australia nef protein (nef) gene, complete cds.
ACCESSION   AF219750
VERSION     AF219750.1
KEYWORDS    .
SOURCE      Human immunodeficiency virus 1 (HIV-1)
  ORGANISM  Human immunodeficiency virus 1
            Viruses; Riboviria; Pararnavirae; Artverviricota; Revtraviricetes;
            Ortervirales; Retroviridae; Orthoretrovirinae; Lentivirus.
REFERENCE   1  (bases 1 to 754)
  AUTHORS   Ashton,L., Rhodes,D., Solomon,A., Deacon,N., Satchell,C., Carr,A.,
            Cooper,D., Biti,R., Stewart,G. and Kaldor,J.
  TITLE     Viral diversity in the nef/LTR region of the HIV-1 genome:
            associations with long-term nonprogression
  JOURNAL   Unpublished
REFERENCE   2  (bases 1 to 754)
  AUTHORS   Rhodes,D.
  TITLE     Direct Submission
  JOURNAL   Submitted (23-DEC-1999) AIDS Molecular Biology Unit, Macfarlane
            Burnet Centre for Medical Research, Yarra Bend Rd., Fairfield,
            Victoria 3078, Australia
FEATURES             Location/Qualifiers
     source          1..754
                     /organism=\"Human immunodeficiency virus 1\"
                     /proviral
                     /mol_type=\"genomic DNA\"
                     /isolate=\"LTS 38d\"
                     /db_xref=\"taxon:11676\"
                     /country=\"Australia\"
     gene            1..657
                     /gene=\"nef\"
     CDS             1..657
                     /gene=\"nef\"
                     /codon_start=1
                     /product=\"nef protein\"
                     /protein_id=\"AAG44221.1\"
                     /translation=\"MGGKWSKRSEDRWSTIRERMRRAPAAEPAADGVGAASRDLEKYG
                     AITSSNTAATNADCAWLEAQEEEEEVGFPVRPQVPLRPMTWKAALDLSHFLKEKGGLE
                     GLVYSQKRRDILDLWIYHTQGFFPDWQNYTPGPGTRFPLTFGWCFKLVPMEREKIEEA
                     NEGENNSLLHPLSQHGMDDPEREVLVWKFDSRLAFHHVARELHPGVLQDLMTPSFYNC
                     \"
ORIGIN      
        1 atgggtggca agtggtcaaa acgtagcgag gatagatggt ctaccataag ggaaagaatg
       61 agacgtgcgc cagcagctga gccagcagca gatggggtgg gagcagcatc tcgagacttg
      121 gaaaaatatg gcgcaatcac aagtagcaat acagcagcta ccaatgctga ttgtgcctgg
      181 ctagaagcac aagaagagga ggaggaggtg ggctttccag tcagacctca agtaccttta
      241 agaccaatga cctggaaggc agctttagat cttagccact ttttaaaaga aaagggggga
      301 ctggaagggc tagtttactc ccaaaaaaga cgagatatcc ttgatttgtg gatctaccac
      361 acacaaggct tcttccctga ttggcaaaac tacacaccag ggccagggac cagatttcca
      421 ctgacctttg ggtggtgctt caagttggta ccaatggagc gagagaaaat agaagaggcc
      481 aatgaaggag agaacaacag tttgttacac cctttaagcc agcatgggat ggatgacccg
      541 gagagagaag tgttagtgtg gaagtttgac agccgcctag catttcatca cgtggctcga
      601 gagctgcatc ccggagtact acaagatctg atgacaccga gcttctacaa ctgctgacat
      661 cggcctttct acagggactc tccgctgggg actctccagg gaggcgtggc ctcggcggga
      721 ctcgggagtg gcgagcctca gatgctgcat ataa";

const RECORD_4: &[u8] =
    b"LOCUS       AB520928                 674 bp    cRNA    linear   VRL 25-JUL-2016
DEFINITION  Influenza A virus (A/Sendai/TU65/2006(H1N1)) M1, M2 genes for
            matrix protein 1, matrix protein 2, partial cds, laboratory strain
            4.
ACCESSION   AB520928
VERSION     AB520928.1
KEYWORDS    .
SOURCE      Influenza A virus (A/Sendai/TU65/2006(H1N1))
  ORGANISM  Influenza A virus (A/Sendai/TU65/2006(H1N1))
            Viruses; Riboviria; Orthornavirae; Negarnaviricota;
            Polyploviricotina; Insthoviricetes; Articulavirales;
            Orthomyxoviridae; Alphainfluenzavirus; Alphainfluenzavirus
            influenzae.
REFERENCE   1
  AUTHORS   Furuse,Y., Suzuki,A., Kishi,M., Nukiwa,N., Shimizu,M., Sawayama,R.,
            Fuji,N. and Oshitani,H.
  TITLE     Occurrence of mixed populations of influenza A viruses that can be
            maintained through transmission in a single host and potential for
            reassortment
  JOURNAL   J. Clin. Microbiol. 48 (2), 369-374 (2010)
   PUBMED   19940049
REFERENCE   2  (bases 1 to 674)
  AUTHORS   Furuse,Y., Suzuki,A. and Oshitani,H.
  TITLE     Direct Submission
  JOURNAL   Submitted (07-SEP-2009) Contact:Yuki Furuse Tohoku University
            Graduate School of Medicine, Department of Virology; Aoba ku
            seiryou chou 2-1, Sendai, Miyagi 980-8575, Japan
FEATURES             Location/Qualifiers
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
                     IFKHGLKRGPSTEGIPESMREEYREEQRNAVDADDDHFVSIELE\"
ORIGIN      
        1 taacattcca tggggccaaa gaaatagcac tcagttattc tgctggtgca cttgccagtt
       61 gtatgggact catatacaac aggatggggg ctgtgaccac cgaatcagca tttggcctta
      121 tatgtgcaac ctgtgaacag attgccgact cccagcataa gtctcacagg caaatggtaa
      181 caacaaccaa tccattaata agacatgaga acagaatggt tctggccagc actacagcta
      241 aggctatgga gcaaatggct ggatcgagcg aacaagcagc tgaggccatg gaggttgcta
      301 gtcaggccag gcagatggtg caggcaatga gagccattgg gactcatcct agctctagca
      361 ctggtctgaa aaatgatctc cttgaaaatt tacaggccta tcagaaacga atgggggtgc
      421 agatgcaacg attcaagtga tcctcttgtt gttgccgcaa gtataattga gattgtgcac
      481 ttgatattgt ggattattga tcgccttttt tccaaaagca tttgtcgtat ctttaaacac
      541 ggtttaaaaa gagggccttc tacggaagga ataccagagt ctatgaggga agaatatcga
      601 gaggaacagc ggaatgctgt ggacgctgac gatgatcatt ttgtcagcat agagctagag
      661 taaaaaacta cctt";

#[test]
#[ignore]
fn test_parse_sequence_record_by_positions() {
    let sequence = parse_sequence_record_by_positions(RECORD);
    // println!("Definition: {:?}", String::from_utf8_lossy(&sequence.taxonomy));

    assert_eq!(sequence.version, b"AB000048.1");
    assert_eq!(
        sequence.definition,
        b"Feline panleukopenia virus gene for nonstructural protein 1, complete cds, isolate: 483."
    );
    assert_eq!(sequence.taxonomy, b"Viruses; Monodnaviria; Shotokuvirae; Cossaviricota; Quintoviricetes; Piccovirales; Parvoviridae; Parvovirinae; Protoparvovirus; Protoparvovirus carnivoran1.");
    assert_eq!(sequence.sequence, b"atgtctggcaaccagtatactgaggaagttatggagggagtaaattggttaaagaaacatgcagaagatgaagcattttcgtttgtttttaaatgtgacaacgtccaactaaatggaaaggatgttcgctggaacaactataccaaaccaattcaaaatgaagagctaacatctttaattagaggagcacaaacagcaatggatcaaaccgaagaagaagaaatggactgggaatcggaagttgatagtctcgccaaaaagcaagtacaaacttttgatgcattaattaaaaaatgtctttttgaagtctttgtttctaaaaatatagaaccaaatgaatgtgtttggtttattcaacatgaatggggaaaagatcaaggctggcattgtcatgttttacttcatagtaagaacttacaacaagcaactggtaaatggctacgcagacaaatgaatatgtattggagtagatggttggtgactctttgttcgataaatttaacaccaactgaaaagattaagctcagagaaattgcagaagatagtgaatgggtaactatattaacatacagacataagcaaacaaaaaaagactatgttaaaatggttcattttggaaatatgatagcatattactttttaacaaagaaaaaaattgtccacatgacaaaagaaagtggctattttttaagtactgattctggttggaaatttaactttatgaagtatcaagacagacatactgtcagcacactttacactgaacaaatgaaaccagaaaccgttgaaaccacagtgacgacagcacaggaaacaaagcgcgggagaattcaaactaaaaaggaagtgtcaatcaaatgtactttgcgggacttggttagtaaaagagtaacatcacctgaagactggatgatgttacaaccagatagttatattgaaatgatggcacaaccaggaggtgaaaatctcttaaaaaatacacttgaaatttgtactttgactttagcaagaacaaaaacagcatttgaattaatacttgaaaaagcagataataccaaactaactaactttgatcttgcaaattctagaacatgtcaaatttttagaatgcacggatggaattggattaaagtttgtcacgctatagcatgtgttttaaatagacaaggtggtaaaagaaatacagttctttttcatggaccagcaagtacaggaaaatctattattgctcaagccatagcacaagctgtgggtaatgttggttgctataatgcagcaaatgtaaattttccatttaatgactgtaccaataaaaatttaatttgggttgaagaagctggtaactttggtcaacaagttaatcaatttaaagcaatttgttctggacaaacaattagaattgatcaaaaaggtaaaggaagtaagcaaattgaaccaactccagtaattatgacaactaatgaaaatataacaattgtaagaattggatgtgaagaaagacctgaacatacacaaccaataagagacagaatgttgaacattaaattagtatgtaagcttccaggagactttggtttggttgataaagaagaatggcctttaatatgtgcatggttagttaaacatggttatcaatcaaccatggctaactacacacatcattggggaaaagtaccagagtgggatgaaaactgggcggagcctaaaatacaagaaggtataaattcaccaggttgcaaagacttagagacacaagcggcaagcaatcctcagagtcaagaccacgttctaactcctctgactccggacgtagtggaccttgcactggaaccgtggagtactccagatacgcctattgcagaaactgcaaatcaacaatcaaaccaacttggcgttactcacaaagacgtgcaagcgagtccgacatggtccgaaatagaggcagacctgagagccatttttacttctgaacaattggaagaagattttcgagacgacttggattaa");
}

#[test]
#[ignore]
fn test_parse_sequence_record() {
    let (sequence, _) = parse_sequence_record(RECORD);
    println!(
        "organism: {:?}",
        String::from_utf8_lossy(&sequence.organism)
    );
    // println!(
    //     "features: {:?}",
    //     String::from_utf8_lossy(&sequence.features)
    // );

    assert_eq!(sequence.version, b"AB000048.1");
    assert_eq!(
        sequence.definition,
        b"Feline panleukopenia virus gene for nonstructural protein 1, complete cds, isolate: 483."
    );
    assert_eq!(sequence.taxonomy, b"Viruses; Monodnaviria; Shotokuvirae; Cossaviricota; Quintoviricetes; Piccovirales; Parvoviridae; Parvovirinae; Protoparvovirus; Protoparvovirus carnivoran1.");
    assert_eq!(sequence.sequence, b"atgtctggcaaccagtatactgaggaagttatggagggagtaaattggttaaagaaacatgcagaagatgaagcattttcgtttgtttttaaatgtgacaacgtccaactaaatggaaaggatgttcgctggaacaactataccaaaccaattcaaaatgaagagctaacatctttaattagaggagcacaaacagcaatggatcaaaccgaagaagaagaaatggactgggaatcggaagttgatagtctcgccaaaaagcaagtacaaacttttgatgcattaattaaaaaatgtctttttgaagtctttgtttctaaaaatatagaaccaaatgaatgtgtttggtttattcaacatgaatggggaaaagatcaaggctggcattgtcatgttttacttcatagtaagaacttacaacaagcaactggtaaatggctacgcagacaaatgaatatgtattggagtagatggttggtgactctttgttcgataaatttaacaccaactgaaaagattaagctcagagaaattgcagaagatagtgaatgggtaactatattaacatacagacataagcaaacaaaaaaagactatgttaaaatggttcattttggaaatatgatagcatattactttttaacaaagaaaaaaattgtccacatgacaaaagaaagtggctattttttaagtactgattctggttggaaatttaactttatgaagtatcaagacagacatactgtcagcacactttacactgaacaaatgaaaccagaaaccgttgaaaccacagtgacgacagcacaggaaacaaagcgcgggagaattcaaactaaaaaggaagtgtcaatcaaatgtactttgcgggacttggttagtaaaagagtaacatcacctgaagactggatgatgttacaaccagatagttatattgaaatgatggcacaaccaggaggtgaaaatctcttaaaaaatacacttgaaatttgtactttgactttagcaagaacaaaaacagcatttgaattaatacttgaaaaagcagataataccaaactaactaactttgatcttgcaaattctagaacatgtcaaatttttagaatgcacggatggaattggattaaagtttgtcacgctatagcatgtgttttaaatagacaaggtggtaaaagaaatacagttctttttcatggaccagcaagtacaggaaaatctattattgctcaagccatagcacaagctgtgggtaatgttggttgctataatgcagcaaatgtaaattttccatttaatgactgtaccaataaaaatttaatttgggttgaagaagctggtaactttggtcaacaagttaatcaatttaaagcaatttgttctggacaaacaattagaattgatcaaaaaggtaaaggaagtaagcaaattgaaccaactccagtaattatgacaactaatgaaaatataacaattgtaagaattggatgtgaagaaagacctgaacatacacaaccaataagagacagaatgttgaacattaaattagtatgtaagcttccaggagactttggtttggttgataaagaagaatggcctttaatatgtgcatggttagttaaacatggttatcaatcaaccatggctaactacacacatcattggggaaaagtaccagagtgggatgaaaactgggcggagcctaaaatacaagaaggtataaattcaccaggttgcaaagacttagagacacaagcggcaagcaatcctcagagtcaagaccacgttctaactcctctgactccggacgtagtggaccttgcactggaaccgtggagtactccagatacgcctattgcagaaactgcaaatcaacaatcaaaccaacttggcgttactcacaaagacgtgcaagcgagtccgacatggtccgaaatagaggcagacctgagagccatttttacttctgaacaattggaagaagattttcgagacgacttggattaa");
}

#[test]
#[ignore]
fn test_parse_sequence_record_2() {
    let (sequence, _) = parse_sequence_record(RECORD_2);
    println!(
        "organism: {:?}",
        String::from_utf8_lossy(&sequence.organism)
    );
    // println!(
    //     "features: {:?}",
    //     String::from_utf8_lossy(&sequence.features)
    // );

    assert_eq!(sequence.version, b"AF148865.1");
    assert_eq!(
        sequence.definition,
        b"Norwalk-like virus strain Gat010-02/97-QC RNA polymerase gene, partial cds."
    );
    assert_eq!(sequence.taxonomy, b"Viruses; Riboviria; Orthornavirae; Pisuviricota; Pisoniviricetes; Picornavirales; Caliciviridae; Norovirus; Norwalk virus.");
    assert_eq!(
        sequence.sequence,
        b"cttctcactctctgtgcgctctctgaagttacaaacttgtcccctgacataatacaggctaattccctcttctccttttat"
    );

    // assert_eq!(sequence.taxonomy, b"AB000048.1");
}

#[test]
fn test_parse_sequence_record_3() {
    let (sequence, _) = parse_sequence_record(RECORD_3);
    // println!("sequence: {:?}", &sequence);
    // println!(
    //     "features: {:?}",
    //     String::from_utf8_lossy(&sequence.features)
    // );

    assert_eq!(sequence.version, b"AF219750.1");
    assert_eq!(
        sequence.definition,
        b"HIV-1 LTS 38d from Australia nef protein (nef) gene, complete cds."
    );
    assert_eq!(sequence.taxonomy, b"Viruses; Riboviria; Pararnavirae; Artverviricota; Revtraviricetes; Ortervirales; Retroviridae; Orthoretrovirinae; Lentivirus.");
    assert_eq!(sequence.sequence, b"atgggtggcaagtggtcaaaacgtagcgaggatagatggtctaccataagggaaagaatgagacgtgcgccagcagctgagccagcagcagatggggtgggagcagcatctcgagacttggaaaaatatggcgcaatcacaagtagcaatacagcagctaccaatgctgattgtgcctggctagaagcacaagaagaggaggaggaggtgggctttccagtcagacctcaagtacctttaagaccaatgacctggaaggcagctttagatcttagccactttttaaaagaaaaggggggactggaagggctagtttactcccaaaaaagacgagatatccttgatttgtggatctaccacacacaaggcttcttccctgattggcaaaactacacaccagggccagggaccagatttccactgacctttgggtggtgcttcaagttggtaccaatggagcgagagaaaatagaagaggccaatgaaggagagaacaacagtttgttacaccctttaagccagcatgggatggatgacccggagagagaagtgttagtgtggaagtttgacagccgcctagcatttcatcacgtggctcgagagctgcatcccggagtactacaagatctgatgacaccgagcttctacaactgctgacatcggcctttctacagggactctccgctggggactctccagggaggcgtggcctcggcgggactcgggagtggcgagcctcagatgctgcatataa");

    assert_eq!(sequence.mol_type, b"genomic DNA");
}

#[test]
#[ignore]
fn test_parse_new_sequence_record() {
    let (sequence, proteins) = parse_new_sequence_record(RECORD_3);
    let flattened_taxonomy: Vec<u8> = sequence
        .taxonomy
        .iter()
        .flat_map(|&slice| slice)
        .copied()
        .collect();
    let flattened_definition: Vec<u8> = sequence
        .definition
        .iter()
        .flat_map(|&slice| slice)
        .copied()
        .collect();

    // for protein in &proteins {
    //     println!("protein_id: {:?}", String::from_utf8(protein.protein_id.as_ref().unwrap().iter().flat_map(|&slice| slice).copied().collect()));
    //     println!("source_id: {:?}", String::from_utf8(protein.source_id.unwrap().to_vec()));
    //     println!("sequence: {:?}", String::from_utf8(protein.sequence.as_ref().unwrap().iter().flat_map(|&slice| slice).copied().collect()));
    //     println!("location: {:?}", String::from_utf8(protein.location.unwrap().to_vec()));
    // }

    assert_eq!(sequence.version.unwrap(), b"AF219750.1" as &[u8]);
    assert_eq!(flattened_taxonomy, b"Viruses; Riboviria; Pararnavirae; Artverviricota; Revtraviricetes; Ortervirales; Retroviridae; Orthoretrovirinae; Lentivirus.");
    assert_eq!(
        flattened_definition,
        b"HIV-1 LTS 38d from Australia nef protein (nef) gene, complete cds."
    );
    assert_eq!(sequence.sequence.unwrap().iter().map(|&&x| x).collect::<Vec<u8>>(), b"atgggtggcaagtggtcaaaacgtagcgaggatagatggtctaccataagggaaagaatgagacgtgcgccagcagctgagccagcagcagatggggtgggagcagcatctcgagacttggaaaaatatggcgcaatcacaagtagcaatacagcagctaccaatgctgattgtgcctggctagaagcacaagaagaggaggaggaggtgggctttccagtcagacctcaagtacctttaagaccaatgacctggaaggcagctttagatcttagccactttttaaaagaaaaggggggactggaagggctagtttactcccaaaaaagacgagatatccttgatttgtggatctaccacacacaaggcttcttccctgattggcaaaactacacaccagggccagggaccagatttccactgacctttgggtggtgcttcaagttggtaccaatggagcgagagaaaatagaagaggccaatgaaggagagaacaacagtttgttacaccctttaagccagcatgggatggatgacccggagagagaagtgttagtgtggaagtttgacagccgcctagcatttcatcacgtggctcgagagctgcatcccggagtactacaagatctgatgacaccgagcttctacaactgctgacatcggcctttctacagggactctccgctggggactctccagggaggcgtggcctcggcgggactcgggagtggcgagcctcagatgctgcatataa");
    assert_eq!(1, proteins.len());
}

#[test]
fn test_parse_new_record_multiple_proteins() {
    let (sequence, proteins) = parse_new_sequence_record(RECORD_4);

    println!("Num proteins: {}", proteins.len());
    for protein in &proteins {
        println!(
            "protein_id: {:?}",
            String::from_utf8(
                protein
                    .protein_id
                    .as_ref()
                    .unwrap()
                    .iter()
                    .flat_map(|&slice| slice)
                    .copied()
                    .collect()
            )
        );
        println!(
            "source_id: {:?}",
            String::from_utf8(protein.source_id.unwrap().to_vec())
        );
        println!(
            "sequence: {:?}",
            String::from_utf8(
                protein
                    .sequence
                    .as_ref()
                    .unwrap()
                    .iter()
                    .flat_map(|&slice| slice)
                    .copied()
                    .collect()
            )
        );
        println!(
            "location: {:?}",
            String::from_utf8(protein.location.unwrap().to_vec())
        );
    }
    assert_eq!(sequence.version.unwrap(), b"AB520928.1" as &[u8]);
}

#[test]
fn test_split_on_delimeter_exclude() {
    let feature_line = b"line1                     /line2                     /line3";
    let separator = b"                     /";
    let lines = split_on_delimiter(feature_line, separator, false);

    let expected_result = Vec::from([b"line1", b"line2", b"line3"]);
    assert_eq!(expected_result, lines);
    // for line in lines {
    //     println!("{:?}", std::str::from_utf8(line).unwrap());
    // }
}

#[test]
fn test_split_on_delimeter_include() {
    let contents = b"\nLOCUSline1\nLOCUSline2\nLOCUSline3";
    let records = split_on_delimiter(contents, b"\nLOCUS", true);

    let expected_result = Vec::from([b"\nLOCUSline1", b"\nLOCUSline2", b"\nLOCUSline3"]);
    assert_eq!(expected_result, records);
}

#[test]
fn test_split_on_delimeter_include_header() {
    let contents = b"HEADER TEXT\nLOCUSline1\nLOCUSline2\nLOCUSline3";
    let records = split_on_delimiter(contents, b"\nLOCUS", true);

    let expected_result = Vec::from([
        b"HEADER TEXT",
        b"\nLOCUSline1",
        b"\nLOCUSline2",
        b"\nLOCUSline3",
    ]);
    assert_eq!(expected_result, records);
}
