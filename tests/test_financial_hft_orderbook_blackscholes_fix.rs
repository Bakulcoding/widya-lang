use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave46_orderbook_l2_and_matching() {
    let code = r#"
        var ob = BukuPesananL2("ETH/USDT")
        orderbook_tambah_pesanan(ob, "b1", "BID", 3000.0, 2.0)
        orderbook_tambah_pesanan(ob, "a1", "ASK", 2990.0, 1.0)

        var depth = orderbook_ambil_kedalaman(ob)
        var cek_depth = depth.best_bid == 3000.0 dan depth.best_ask == 2990.0

        var trades = orderbook_cocokkan(ob)
        var cek_trade = panjang(trades) == 1 dan trades[0].jumlah_eksekusi == 1.0

        kembalikan cek_depth dan cek_trade
    "#;
    let res = jalankan(code).expect("Orderbook test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave46_financial_indicators() {
    let code = r#"
        var harga = [10.0, 11.0, 12.0, 13.0, 14.0, 15.0]
        var vol = [100.0, 100.0, 100.0, 100.0, 100.0, 100.0]

        var ema = indikator_ema(harga, 3)
        var cek_ema = panjang(ema) == 6 dan ema[5] > 13.0

        var rsi = indikator_rsi(harga, 5)
        var cek_rsi = rsi > 80.0 // Tren naik terus

        var vwap = indikator_vwap(harga, vol)
        var cek_vwap = vwap == 12.5

        kembalikan cek_ema dan cek_rsi dan cek_vwap
    "#;
    let res = jalankan(code).expect("Indicators test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave46_black_scholes_and_greeks() {
    let code = r#"
        var c = OpsiEropa("CALL", 100.0, 100.0, 1.0, 0.05, 0.2)
        var p_call = opsi_hitung_harga(c)
        var cek_p_call = p_call > 9.0 dan p_call < 12.0

        var g = opsi_hitung_greeks(c)
        var cek_greeks = g.delta > 0.5 dan g.gamma > 0.0 dan g.vega > 0.0

        var p = OpsiEropa("PUT", 100.0, 100.0, 1.0, 0.05, 0.2)
        var p_put = opsi_hitung_harga(p)
        var cek_p_put = p_put > 5.0 dan p_put < 7.0

        kembalikan cek_p_call dan cek_greeks dan cek_p_put
    "#;
    let res = jalankan(code).expect("Black Scholes test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave46_fix_protocol_engine() {
    let code = r#"
        var fix = PesanFIX("D", 1, "CLIENT", "SERVER")
        fix_tambah_tag(fix, "55", "BTCUSD")
        fix_tambah_tag(fix, "38", "5")

        var raw = fix_enkode(fix)
        var cek_raw = panjang(raw) > 20

        var dec = fix_dekode(raw)
        var cek_dec = dec.tags["55"] == "BTCUSD" dan dec.tags["38"] == "5"

        kembalikan cek_raw dan cek_dec
    "#;
    let res = jalankan(code).expect("FIX Protocol test failed");
    assert_eq!(res, Value::Bool(true));
}
