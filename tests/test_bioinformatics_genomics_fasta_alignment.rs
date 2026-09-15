use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave47_fasta_fastq_parser() {
    let code = r#"
        var fasta_str = ">gene1\nATGC\n>gene2\nGGCC\n"
        var fa = ParserFASTA(fasta_str)
        var cek_fa = fa.total_record == 2 dan fa.records[0].sekuens == "ATGC"

        var fastq_str = "@R1\nAT\n+\nII\n"
        var fq = ParserFASTQ(fastq_str)
        var cek_fq = fq.total_reads == 1 dan fq.reads[0].id == "R1"

        var gc = genom_hitung_gc_content("ATGC")
        var cek_gc = gc == 50.0

        kembalikan cek_fa dan cek_fq dan cek_gc
    "#;
    let res = jalankan(code).expect("FASTA/FASTQ test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave47_dna_rna_protein_translation() {
    let code = r#"
        var dna = "ATGGCC"
        var rna = dna_transkripsi_ke_rna(dna)
        var cek_rna = rna == "AUGGCC"

        var rev = dna_komplemen_terbalik("ATGC")
        var cek_rev = rev == "GCAT"

        var prot = rna_translasi_ke_protein("AUGGCCUAA")
        var cek_prot = prot == "MA*"

        kembalikan cek_rna dan cek_rev dan cek_prot
    "#;
    let res = jalankan(code).expect("Translation test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave47_sequence_alignment() {
    let code = r#"
        var aligner = PenjajarSekuens(2.0, -1.0, -2.0)
        var g_res = penjajaran_global(aligner, "GATTACA", "GCATGCU")
        var cek_global = g_res.skor_keselarasan == 2.0

        var l_res = penjajaran_lokal(aligner, "AAAGATTACATTT", "CCCGATTACAGGG")
        var cek_local = l_res.skor_keselarasan == 14.0 // 7 match * 2 = 14

        kembalikan cek_global dan cek_local
    "#;
    let res = jalankan(code).expect("Alignment test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave47_kmer_and_hamming() {
    let code = r#"
        var kmers = kmer_hitung_frekuensi("ATATAT", 2)
        var cek_kmer = kmers["AT"] == 3 dan kmers["TA"] == 2

        var ham = genom_jarak_hamming("GAGC", "CATC")
        var cek_ham = ham == 2

        kembalikan cek_kmer dan cek_ham
    "#;
    let res = jalankan(code).expect("Kmer and Hamming test failed");
    assert_eq!(res, Value::Bool(true));
}
