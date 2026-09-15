use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave54_hdl_rtl_synthesis() {
    let code = r#"
        var modul = ModulRTL("ALU8Bit");
        rtl_tambah_port(modul, "clk", "input", 1);
        rtl_tambah_port(modul, "a", "input", 8);
        rtl_tambah_port(modul, "b", "input", 8);
        rtl_tambah_port(modul, "out", "output", 8);
        rtl_tambah_penugasan(modul, "out_sum", "a + b");
        rtl_tambah_blok_sekuensial(modul, "posedge clk", "out <= out_sum;");

        var verilog = rtl_ekspor_verilog(modul);
        var vhdl = rtl_ekspor_vhdl(modul);

        var cek_verilog = panjang(verilog) > 20;
        var cek_vhdl = panjang(vhdl) > 20;

        kembalikan cek_verilog dan cek_vhdl;
    "#;
    let res = jalankan(code).expect("RTL Synthesis test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave54_gate_netlist_and_cycle_sim() {
    let code = r#"
        var netlist = NetlistGerbang("XOR_Gate");
        netlist_sisip_gerbang(netlist, "G1", "XOR", ["in1", "in2"], "out1");
        
        var stim1 = {"in1": 1, "in2": 0};
        var res1 = netlist_simulasi_siklus(netlist, stim1);
        
        var stim2 = {"in1": 1, "in2": 1};
        var res2 = netlist_simulasi_siklus(netlist, stim2);

        var cek1 = res1["out1"] == 1.0;
        var cek2 = res2["out1"] == 0.0;

        kembalikan cek1 dan cek2;
    "#;
    let res = jalankan(code).expect("Gate Netlist Simulation test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave54_sta_and_vcd_waveform() {
    let code = r#"
        var sta = MesinSTA(500.0);
        sta_tambah_lintasan(sta, "Path1", "reg1", "reg2", 800.0);
        sta_tambah_lintasan(sta, "Path2_Crit", "reg1", "reg3", 1200.0);
        var sta_res = sta_hitung_lintasan_kritis(sta);

        var cek_sta = sta_res["tunda_kritis_ps"] == 1200.0 dan sta_res["timing_terpenuhi"] == benar;

        var vcd = PerekamGelombangVCD("test_tb", "1ps");
        vcd_catat_transisi(vcd, 0, "clk", 0);
        vcd_catat_transisi(vcd, 1000, "clk", 1);
        var vcd_doc = vcd_hasilkan_dokumen(vcd);

        var cek_vcd = panjang(vcd_doc) > 20;

        kembalikan cek_sta dan cek_vcd;
    "#;
    let res = jalankan(code).expect("STA & VCD Waveform test failed");
    assert_eq!(res, Value::Bool(true));
}
