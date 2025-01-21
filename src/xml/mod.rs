pub fn get_download_link_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n");
    for line in splited {
        if line.trim().starts_with("<FtpPath_GenBank>") {
            let ftp_link = line
                .replace("<FtpPath_GenBank>", "")
                .replace("</FtpPath_GenBank>", "")
                .replace("ftp://", "");
            let last = ftp_link.split("/").last();
            return match last {
                None => Err("Nenhum link encontrado".parse().unwrap()),
                Some(value) => {
                    let ftp_link_download = "https://".to_owned()
                        + ftp_link.clone().trim()
                        + "/"
                        + value
                        + "_genomic.fna.gz";
                    Ok(ftp_link_download)
                }
            };
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

pub fn get_id_list_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n");
    for line in splited {
        if line.starts_with("<Id>") {
            return Ok(line.replace("<Id>", "").replace("</Id>", ""));
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

pub fn get_id_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n").collect::<Vec<_>>();
    for i in 0..splited.len() {
        if splited[i].starts_with("<Id>") {
            if splited[i + 1].starts_with("<Id>") {
                return Ok(splited[i + 1].replace("<Id>", "").replace("</Id>", ""));
            }
            return Ok(splited[i].replace("<Id>", "").replace("</Id>", ""));
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

pub fn get_bioproject_accn_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n");
    for line in splited {
        if line.trim().starts_with("<BioprojectAccn>") {
            return Ok(String::from(
                line.replace("<BioprojectAccn>", "")
                    .replace("</BioprojectAccn>", "")
                    .trim(),
            ));
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

pub fn get_count_from_xml(xml: String) -> Result<u32, String> {
    let splited = xml.split("\n");
    for line in splited {
        if line.trim().starts_with("<Count>") {
            return Ok(line
                .replace("<Count>", "")
                .replace("</Count>", "")
                .trim()
                .parse()
                .unwrap());
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

pub fn get_run_from_xml(xml: String) -> Result<String, String> {
    let splited = xml.split("\n");
    for line in splited {
        let line = line.trim();
        if let Some(start_index) = line.find("Run acc=\"") {
            let substring = &line[start_index + 9..];
            if let Some(end_index) = substring.find('"') {
                let run_acc = &substring[..end_index];
                return Ok(run_acc.to_string());
            }
        }
    }
    Err(String::from("Falha ao parsear arquivo"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bioproject_accn_from_xml() {
        let xml = r#"
            <?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE eSummaryResult PUBLIC "-//NLM//DTD esummary assembly 20230126//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20230126/esummary_assembly.dtd">
<eSummaryResult>
    <DocumentSummarySet status="OK">
        <DbBuild>Build250120-0920.1</DbBuild>
        <DocumentSummary uid="538048">
            <RsUid>538048</RsUid>
            <GbUid>538028</GbUid>
            <AssemblyAccession>GCF_000195955.2</AssemblyAccession>
            <LastMajorReleaseAccession>GCF_000195955.2</LastMajorReleaseAccession>
            <LatestAccession></LatestAccession>
            <ChainId>195955</ChainId>
            <AssemblyName>ASM19595v2</AssemblyName>
            <UCSCName></UCSCName>
            <EnsemblName></EnsemblName>
            <Taxid>83332</Taxid>
            <Organism>Mycobacterium tuberculosis H37Rv (high G+C Gram-positive bacteria)</Organism>
            <SpeciesTaxid>1773</SpeciesTaxid>
            <SpeciesName>Mycobacterium tuberculosis</SpeciesName>
            <AssemblyType>haploid</AssemblyType>
            <AssemblyStatus>Complete Genome</AssemblyStatus>
            <AssemblyStatusSort>1</AssemblyStatusSort>
            <WGS></WGS>
            <GB_BioProjects>
                <Bioproj>
                    <BioprojectAccn>PRJNA224</BioprojectAccn>
                    <BioprojectId>224</BioprojectId>
                </Bioproj>
            </GB_BioProjects>
            <GB_Projects>
	</GB_Projects>
            <RS_BioProjects>
                <Bioproj>
                    <BioprojectAccn>PRJNA57777</BioprojectAccn>
                    <BioprojectId>57777</BioprojectId>
                </Bioproj>
            </RS_BioProjects>
            <RS_Projects>
	</RS_Projects>
            <BioSampleAccn>SAMEA3138326</BioSampleAccn>
            <BioSampleId>3215772</BioSampleId>
            <Biosource>
                <InfraspeciesList>
                    <Infraspecie>
                        <Sub_type>strain</Sub_type>
                        <Sub_value>H37Rv</Sub_value>
                    </Infraspecie>
                </InfraspeciesList>
                <Sex></Sex>
                <Isolate></Isolate>
            </Biosource>
            <Coverage></Coverage>
            <PartialGenomeRepresentation>false</PartialGenomeRepresentation>
            <Primary>538038</Primary>
            <AssemblyDescription></AssemblyDescription>
            <ReleaseLevel>Major</ReleaseLevel>
            <ReleaseType>Major</ReleaseType>
            <AsmReleaseDate_GenBank>2013/02/08 00:00</AsmReleaseDate_GenBank>
            <AsmReleaseDate_RefSeq>2013/02/08 00:00</AsmReleaseDate_RefSeq>
            <SeqReleaseDate>2013/02/01 00:00</SeqReleaseDate>
            <AsmUpdateDate>2013/02/08 00:00</AsmUpdateDate>
            <SubmissionDate>2013/02/01 00:00</SubmissionDate>
            <LastUpdateDate>2013/02/08 00:00</LastUpdateDate>
            <SubmitterOrganization>Sanger Institute</SubmitterOrganization>
            <RefSeq_category>reference genome</RefSeq_category>
            <AnomalousList>
	</AnomalousList>
            <ExclFromRefSeq>
	</ExclFromRefSeq>
            <PropertyList>
                <string>from-type</string>
                <string>full-genome-representation</string>
                <string>genbank_has_annotation</string>
                <string>has-chromosome</string>
                <string>has_annotation</string>
                <string>latest</string>
                <string>latest_genbank</string>
                <string>latest_refseq</string>
                <string>reference</string>
                <string>refseq_has_annotation</string>
            </PropertyList>
            <FromType>assembly from type material</FromType>
            <Synonym>
                <Genbank>GCA_000195955.2</Genbank>
                <RefSeq>GCF_000195955.2</RefSeq>
                <Similarity>identical</Similarity>
            </Synonym>
            <ContigN50>4411532</ContigN50>
            <ScaffoldN50>4411532</ScaffoldN50>
            <AnnotRptUrl></AnnotRptUrl>
            <FtpPath_GenBank>ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/195/955/GCA_000195955.2_ASM19595v2</FtpPath_GenBank>
            <FtpPath_RefSeq>ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/195/955/GCF_000195955.2_ASM19595v2</FtpPath_RefSeq>
            <FtpPath_Assembly_rpt>ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/195/955/GCF_000195955.2_ASM19595v2/GCF_000195955.2_ASM19595v2_assembly_report.txt</FtpPath_Assembly_rpt>
            <FtpPath_Stats_rpt>ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/195/955/GCF_000195955.2_ASM19595v2/GCF_000195955.2_ASM19595v2_assembly_stats.txt</FtpPath_Stats_rpt>
            <FtpPath_Regions_rpt></FtpPath_Regions_rpt>
            <Busco>
                <RefSeqAnnotationRelease></RefSeqAnnotationRelease>
                <BuscoLineage></BuscoLineage>
                <BuscoVer></BuscoVer>
                <Complete></Complete>
                <SingleCopy></SingleCopy>
                <Duplicated></Duplicated>
                <Fragmented></Fragmented>
                <Missing></Missing>
                <TotalCount>0</TotalCount>
            </Busco>
            <SortOrder>1C1T12519955884670001959559797</SortOrder>
            <Meta>
                <![CDATA[ <Stats><Stat category="alt_loci_count" sequence_tag="all">0</Stat><Stat category="chromosome_count" sequence_tag="all">1</Stat><Stat category="contig_count" sequence_tag="all">1</Stat><Stat category="contig_l50" sequence_tag="all">1</Stat><Stat category="contig_n50" sequence_tag="all">4411532</Stat><Stat category="non_chromosome_replicon_count" sequence_tag="all">0</Stat><Stat category="replicon_count" sequence_tag="all">1</Stat><Stat category="scaffold_count" sequence_tag="all">1</Stat><Stat category="scaffold_count" sequence_tag="placed">1</Stat><Stat category="scaffold_count" sequence_tag="unlocalized">0</Stat><Stat category="scaffold_count" sequence_tag="unplaced">0</Stat><Stat category="scaffold_l50" sequence_tag="all">1</Stat><Stat category="scaffold_n50" sequence_tag="all">4411532</Stat><Stat category="total_length" sequence_tag="all">4411532</Stat><Stat category="ungapped_length" sequence_tag="all">4411532</Stat></Stats><FtpSites><FtpPath type="Assembly_rpt">ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/195/955/GCF_000195955.2_ASM19595v2/GCF_000195955.2_ASM19595v2_assembly_report.txt</FtpPath><FtpPath type="GenBank">ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/195/955/GCA_000195955.2_ASM19595v2</FtpPath><FtpPath type="RefSeq">ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/195/955/GCF_000195955.2_ASM19595v2</FtpPath><FtpPath type="Stats_rpt">ftp://ftp.ncbi.nlm.nih.gov/genomes/all/GCF/000/195/955/GCF_000195955.2_ASM19595v2/GCF_000195955.2_ASM19595v2_assembly_stats.txt</FtpPath></FtpSites><assembly-level>90</assembly-level><assembly-status>Complete Genome</assembly-status><representative-status>reference genome</representative-status><submitter-organization>Sanger Institute</submitter-organization><taxonomy-check-status>OK</taxonomy-check-status>    ]]>
    </Meta>
</DocumentSummary>
</DocumentSummarySet>
</eSummaryResult>
        "#;
        assert_eq!(
            get_bioproject_accn_from_xml(xml.to_string()).unwrap(),
            "PRJNA224"
        );
    }

    #[test]
    fn test_get_count_from_xml() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE eSearchResult PUBLIC "-//NLM//DTD esearch 20060628//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20060628/esearch.dtd">
<eSearchResult>
    <Count>0</Count>
    <RetMax>0</RetMax>
    <RetStart>0</RetStart>
    <IdList/>
    <TranslationSet/>
    <QueryTranslation>(PRJNA57777[All Fields])</QueryTranslation>
    <ErrorList>
        <PhraseNotFound>PRJNA57777</PhraseNotFound>
    </ErrorList>
    <WarningList>
        <OutputMessage>No items found.</OutputMessage>
    </WarningList>
</eSearchResult>"#;

        assert_eq!(get_count_from_xml(xml.to_string()).unwrap(), 0);
    }

    #[test]
    fn test_get_run_from_xml() {
        let xml = r#"
        <?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE eSummaryResult PUBLIC "-//NLM//DTD esummary v1 20041029//EN" "https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20041029/esummary-v1.dtd">
<eSummaryResult>
    <DocSum>
        <Id>25696663</Id>
        <Item Name="ExpXml" Type="String">&lt;Summary&gt;&lt;Title&gt;Mycobacterium lepromatosis strain:FJ924&lt;/Title&gt;&lt;Platform instrument_model="Illumina HiSeq 2000"&gt;ILLUMINA&lt;/Platform&gt;&lt;Statistics total_runs="1" total_spots="69365256" total_bases="10543518912" total_size="6189469256" load_done="true" cluster_name="public"/&gt;&lt;/Summary&gt;&lt;Submitter acc="SRA1557126" center_name="UT MD. Anderson Cancer Center" contact_name="Xiaofeng Zheng" lab_name="Bioinformatics &amp;amp; Computational Biology"/&gt;&lt;Experiment acc="SRX18637364" ver="1" status="public" name="Mycobacterium lepromatosis strain:FJ924"/&gt;&lt;Study acc="SRP412366" name="Mycobacterium lepromatosis strain:FJ924 Genome sequencing"/&gt;&lt;Organism taxid="480418" ScientificName="Mycobacterium lepromatosis"/&gt;&lt;Sample acc="SRS16082914" name=""/&gt;&lt;Instrument ILLUMINA="Illumina HiSeq 2000"/&gt;&lt;Library_descriptor&gt;&lt;LIBRARY_NAME&gt;FJ924&lt;/LIBRARY_NAME&gt;&lt;LIBRARY_STRATEGY&gt;WGS&lt;/LIBRARY_STRATEGY&gt;&lt;LIBRARY_SOURCE&gt;GENOMIC&lt;/LIBRARY_SOURCE&gt;&lt;LIBRARY_SELECTION&gt;RANDOM&lt;/LIBRARY_SELECTION&gt;&lt;LIBRARY_LAYOUT&gt; &lt;PAIRED/&gt; &lt;/LIBRARY_LAYOUT&gt;&lt;/Library_descriptor&gt;&lt;Bioproject&gt;PRJNA281005&lt;/Bioproject&gt;&lt;Biosample&gt;SAMN03481272&lt;/Biosample&gt;</Item>
        <Item Name="Runs" Type="String">&lt;Run acc="SRR22674481" total_spots="69365256" total_bases="10543518912" load_done="true" is_public="true" cluster_name="public" static_data_available="true"/&gt;</Item>
        <Item Name="ExtLinks" Type="String"></Item>
        <Item Name="CreateDate" Type="String">2022/12/11</Item>
        <Item Name="UpdateDate" Type="String">2022/12/11</Item>
    </DocSum>
</eSummaryResult>
        "#;
        assert_eq!(get_run_from_xml(xml.to_string()).unwrap(), "SRR22674481");
    }
}
