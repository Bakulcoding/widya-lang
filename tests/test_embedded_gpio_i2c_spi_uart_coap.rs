use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave41_embedded_gpio_and_pwm() {
    let code = r#"
        var pin = PinGPIO(13, "Output")
        gpio_tulis(pin, 1)
        var cek_high = gpio_baca(pin) == 1

        gpio_tulis(pin, 0)
        var cek_low = gpio_baca(pin) == 0

        gpio_set_pwm(pin, 5000.0, 50.0)
        var cek_pwm = pin.pwm_frekuensi_hz == 5000.0 dan pin.pwm_duty_cycle == 50.0

        kembalikan cek_high dan cek_low dan cek_pwm
    "#;
    let res = jalankan(code).expect("GPIO test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave41_hardware_bus_i2c_and_spi() {
    let code = r#"
        var i2c = BusI2C(1, 0x68)
        var bytes_wr = i2c_tulis_register(i2c, 0x6B, [0x01, 0x02])
        var cek_wr = bytes_wr == 2

        var res_rd = i2c_baca_register(i2c, 0x6B, 2)
        var cek_rd = panjang(res_rd) == 2 dan res_rd[0] == 0x01 dan res_rd[1] == 0x02

        var spi = BusSPI(10, 0, 1000000)
        var rx = spi_transfer(spi, [0xAA, 0x55])
        var cek_spi = panjang(rx) == 2

        kembalikan cek_wr dan cek_rd dan cek_spi
    "#;
    let res = jalankan(code).expect("I2C and SPI test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave41_serial_uart_and_crc16() {
    let code = r#"
        var uart = PortSerialUART("COM1", 9600)
        var sent = uart_kirim_teks(uart, "HELLO_EMBEDDED")
        var cek_sent = sent == 14 dan uart.total_terkirim == 14

        var text = uart_baca_tersedia(uart)
        var cek_rx = text == "HELLO_EMBEDDED"

        var crc = uart_hitung_crc16("123456789")
        var cek_crc = crc > 0

        kembalikan cek_sent dan cek_rx dan cek_crc
    "#;
    let res = jalankan(code).expect("UART test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave41_coap_iot_protocol() {
    let code = r#"
        var req = PesanCoAP("CON", "POST", 5555, "tkn9", "/aktuator/relay")
        var cek_req = req.tipe_pesan == "CON" dan req.kode_metode == "POST" dan req.id_pesan == 5555

        var biner = coap_enkode(req)
        var cek_biner = panjang(biner) > 0

        var decoded = coap_dekode(biner)
        var cek_dec = decoded.tipe_pesan == "CON" dan decoded.kode_metode == "POST" dan decoded.id_pesan == 5555 dan decoded.token == "tkn9" dan decoded.uri_path == "/aktuator/relay"

        kembalikan cek_req dan cek_biner dan cek_dec
    "#;
    let res = jalankan(code).expect("CoAP test failed");
    assert_eq!(res, Value::Bool(true));
}
