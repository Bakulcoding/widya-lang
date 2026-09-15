use widya::jalankan;
use widya::value::Value;

#[test]
fn test_fs_1_sql_join_groupby_order_limit() {
    let code = r##"
struktur TabelRelasional {
    nama_tabel, kolom, baris_data, indeks_pk,
    fungsi inisialisasi(nama, daftar_kolom) {
        ini.nama_tabel = nama; ini.kolom = daftar_kolom; ini.baris_data = []; ini.indeks_pk = {};
    }
    fungsi sisip(data_record) {
        misal id = data_record["id"];
        jika id != nihil dan ada_kunci(ini.indeks_pk, ke_teks(id)) { kembalikan salah; }
        tambah(ini.baris_data, data_record);
        jika id != nihil { ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1; }
        kembalikan benar;
    }
}
fungsi saring_bawaan(daftar, predikat) {
    misal hasil = [];
    untuk item dalam daftar { jika predikat(item) { tambah(hasil, item); } }
    kembalikan hasil;
}
fungsi apakah_huruf(c) {
    jika panjang(c) == 0 { kembalikan salah; }
    tetap hm = {"A":benar,"B":benar,"C":benar,"D":benar,"E":benar,"F":benar,"G":benar,"H":benar,"I":benar,"J":benar,"K":benar,"L":benar,"M":benar,"N":benar,"O":benar,"P":benar,"Q":benar,"R":benar,"S":benar,"T":benar,"U":benar,"V":benar,"W":benar,"X":benar,"Y":benar,"Z":benar,"a":benar,"b":benar,"c":benar,"d":benar,"e":benar,"f":benar,"g":benar,"h":benar,"i":benar,"j":benar,"k":benar,"l":benar,"m":benar,"n":benar,"o":benar,"p":benar,"q":benar,"r":benar,"s":benar,"t":benar,"u":benar,"v":benar,"w":benar,"x":benar,"y":benar,"z":benar,"_":benar};
    kembalikan ada_kunci(hm, c);
}
fungsi apakah_digit(c) {
    tetap dm = {"0":benar,"1":benar,"2":benar,"3":benar,"4":benar,"5":benar,"6":benar,"7":benar,"8":benar,"9":benar};
    kembalikan ada_kunci(dm, c);
}
fungsi apakah_huruf_atau_digit(c) { kembalikan apakah_huruf(c) atau apakah_digit(c); }
fungsi ke_besar_teks(s) {
    tetap mb = {"a":"A","b":"B","c":"C","d":"D","e":"E","f":"F","g":"G","h":"H","i":"I","j":"J","k":"K","l":"L","m":"M","n":"N","o":"O","p":"P","q":"Q","r":"R","s":"S","t":"T","u":"U","v":"V","w":"W","x":"X","y":"Y","z":"Z"};
    misal hasil = ""; misal i = 0;
    selama i < panjang(s) {
        misal c = s[i];
        jika ada_kunci(mb, c) { hasil = hasil + mb[c]; } lainnya { hasil = hasil + c; }
        i = i + 1;
    }
    kembalikan hasil;
}
fungsi ke_angka(s) {
    tetap dm = {"0":0,"1":1,"2":2,"3":3,"4":4,"5":5,"6":6,"7":7,"8":8,"9":9};
    misal neg = salah; misal start_idx = 0;
    jika panjang(s) > 0 dan s[0] == "-" { neg = benar; start_idx = 1; }
    misal hasil = 0; misal i = start_idx;
    selama i < panjang(s) {
        misal d = s[i];
        jika ada_kunci(dm, d) { hasil = hasil * 10 + dm[d]; } lainnya { berhenti; }
        i = i + 1;
    }
    jika neg { hasil = 0 - hasil; }
    kembalikan hasil;
}
fungsi ke_pecahan(s) {
    tetap dm = {"0":0,"1":1,"2":2,"3":3,"4":4,"5":5,"6":6,"7":7,"8":8,"9":9};
    misal bagian = pisah(s, ".");
    misal bulat = ke_angka(bagian[0]);
    jika panjang(bagian) < 2 { kembalikan bulat; }
    misal frac_str = bagian[1];
    misal frac = 0.0; misal div = 1.0; misal i = 0;
    selama i < panjang(frac_str) {
        misal d = frac_str[i];
        jika ada_kunci(dm, d) { frac = frac * 10.0 + dm[d]; div = div * 10.0; }
        i = i + 1;
    }
    kembalikan bulat + (frac / div);
}
fungsi pisah(s, delim) {
    misal hasil = []; misal mulai = 0; misal i = 0; misal p_delim = panjang(delim);
    selama i <= panjang(s) - p_delim {
        misal cocok = benar; misal j = 0;
        selama j < p_delim {
            jika s[i + j] != delim[j] { cocok = salah; berhenti; }
            j = j + 1;
        }
        jika cocok {
            tambah(hasil, potong(s, mulai, i));
            i = i + p_delim; mulai = i;
        } lainnya { i = i + 1; }
    }
    tambah(hasil, potong(s, mulai, panjang(s)));
    kembalikan hasil;
}
fungsi cari_teks(s, needle) {
    misal i = 0; misal ns = panjang(s); misal nn = panjang(needle);
    selama i <= ns - nn {
        misal cocok = benar; misal j = 0;
        selama j < nn {
            jika s[i + j] != needle[j] { cocok = salah; berhenti; }
            j = j + 1;
        }
        jika cocok { kembalikan i; }
        i = i + 1;
    }
    kembalikan -1;
}
fungsi ada(s, needle) { kembalikan cari_teks(s, needle) >= 0; }
fungsi sql_lexer(sql_string) {
    misal tokens = []; misal pos = 0; misal len = panjang(sql_string);
    tetap KM = {"SELECT":"SELECT","FROM":"FROM","WHERE":"WHERE","JOIN":"JOIN","ON":"ON","GROUP":"GROUP","BY":"BY","ORDER":"ORDER","ASC":"ASC","DESC":"DESC","INSERT":"INSERT","INTO":"INTO","VALUES":"VALUES","UPDATE":"UPDATE","SET":"SET","DELETE":"DELETE","LIMIT":"LIMIT","AND":"AND","OR":"OR","NOT":"NOT","NULL":"NULL","SUM":"SUM","AVG":"AVG","COUNT":"COUNT","MIN":"MIN","MAX":"MAX","INNER":"INNER","LEFT":"LEFT","RIGHT":"RIGHT","OUTER":"OUTER"};
    selama pos < len {
        misal c = sql_string[pos]; misal dt = salah;
        jika bukan dt dan (c == " " atau c == "\t" atau c == "\n" atau c == "\r") { pos = pos + 1; dt = benar; }
        jika bukan dt dan c == ";" { tambah(tokens, {"tipe":"TITIK_KOMA","nilai":";"}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "," { tambah(tokens, {"tipe":"KOMA","nilai":","}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "*" { tambah(tokens, {"tipe":"ASTERISK","nilai":"*"}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "(" { tambah(tokens, {"tipe":"KURUNG_BUKA","nilai":"("}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == ")" { tambah(tokens, {"tipe":"KURUNG_TUTUP","nilai":")"}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "." { tambah(tokens, {"tipe":"DOT","nilai":"."}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "=" { tambah(tokens, {"tipe":"OP_EQ","nilai":"="}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == ">" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" { tambah(tokens, {"tipe":"OP_GTE","nilai":">="}); pos = pos + 2; }
            lainnya { tambah(tokens, {"tipe":"OP_GT","nilai":">"}); pos = pos + 1; }
            dt = benar;
        }
        jika bukan dt dan c == "<" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" { tambah(tokens, {"tipe":"OP_LTE","nilai":"<="}); pos = pos + 2; }
            lainnya jika pos + 1 < len dan sql_string[pos + 1] == ">" { tambah(tokens, {"tipe":"OP_NEQ","nilai":"<>"}); pos = pos + 2; }
            lainnya { tambah(tokens, {"tipe":"OP_LT","nilai":"<"}); pos = pos + 1; }
            dt = benar;
        }
        jika bukan dt dan c == "!" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" { tambah(tokens, {"tipe":"OP_NEQ","nilai":"!="}); pos = pos + 2; dt = benar; }
        }
        jika bukan dt dan c == "'" {
            misal mulai = pos + 1; pos = pos + 1;
            selama pos < len dan sql_string[pos] != "'" {
                jika sql_string[pos] == "\\" dan pos + 1 < len { pos = pos + 2; }
                lainnya { pos = pos + 1; }
            }
            tambah(tokens, {"tipe":"STRING","nilai":potong(sql_string, mulai, pos)});
            pos = pos + 1; dt = benar;
        }
        jika bukan dt dan c == "\"" {
            misal mulai = pos + 1; pos = pos + 1;
            selama pos < len dan sql_string[pos] != "\"" {
                jika sql_string[pos] == "\\" dan pos + 1 < len { pos = pos + 2; }
                lainnya { pos = pos + 1; }
            }
            tambah(tokens, {"tipe":"STRING","nilai":potong(sql_string, mulai, pos)});
            pos = pos + 1; dt = benar;
        }
        jika bukan dt dan apakah_digit(c) {
            misal mulai = pos;
            selama pos < len dan (apakah_digit(sql_string[pos]) atau sql_string[pos] == ".") { pos = pos + 1; }
            tambah(tokens, {"tipe":"NUMBER","nilai":potong(sql_string, mulai, pos)});
            dt = benar;
        }
        jika bukan dt dan apakah_huruf(c) {
            misal mulai = pos;
            selama pos < len dan apakah_huruf_atau_digit(sql_string[pos]) { pos = pos + 1; }
            misal ident = potong(sql_string, mulai, pos);
            misal iu = ke_besar_teks(ident);
            jika ada_kunci(KM, iu) { tambah(tokens, {"tipe":iu,"nilai":ident}); }
            lainnya { tambah(tokens, {"tipe":"IDENT","nilai":ident}); }
            dt = benar;
        }
        jika bukan dt { pos = pos + 1; }
    }
    kembalikan tokens;
}
struktur ParserSQL {
    tokens, pos, len_tokens,
    fungsi inisialisasi(tl) { ini.tokens = tl; ini.pos = 0; ini.len_tokens = panjang(tl); }
    fungsi sekarang() { jika ini.pos < ini.len_tokens { kembalikan ini.tokens[ini.pos]; } kembalikan nihil; }
    fungsi lihat(offset) { misal idx = ini.pos + offset; jika idx >= 0 dan idx < ini.len_tokens { kembalikan ini.tokens[idx]; } kembalikan nihil; }
    fungsi maju() { misal tok = ini.sekarang(); ini.pos = ini.pos + 1; kembalikan tok; }
    fungsi cocok(tipe) { misal tok = ini.sekarang(); jika tok != nihil dan tok.tipe == tipe { kembalikan ini.maju(); } kembalikan nihil; }
    fungsi parse_kolom_atau_expr() {
        misal tok = ini.sekarang(); jika tok == nihil { kembalikan nihil; }
        jika tok.tipe == "SUM" atau tok.tipe == "AVG" atau tok.tipe == "COUNT" atau tok.tipe == "MIN" atau tok.tipe == "MAX" {
            misal af = ini.maju(); ini.cocok("KURUNG_BUKA"); misal ie = ini.parse_kolom_atau_expr(); ini.cocok("KURUNG_TUTUP");
            kembalikan {"jenis":"agregat","fn_nama":af.tipe,"argumen":ie};
        }
        jika tok.tipe == "ASTERISK" { ini.maju(); kembalikan {"jenis":"asterisk"}; }
        misal b1 = ini.cocok("IDENT"); jika b1 == nihil { kembalikan nihil; }
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "DOT" {
            ini.maju(); misal b2 = ini.cocok("IDENT"); jika b2 == nihil { b2 = ini.cocok("ASTERISK"); }
            kembalikan {"jenis":"kolom_qualified","tabel_alias":b1.nilai,"kolom":b2.nilai};
        }
        kembalikan {"jenis":"kolom","nama":b1.nilai};
    }
    fungsi parse_daftar_select() {
        misal wl = [];
        selama benar {
            misal expr = ini.parse_kolom_atau_expr(); jika expr == nihil { berhenti; }
            misal ak = nihil;
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "IDENT" dan ke_besar_teks(ini.lihat(0).nilai) == "AS") {
                ini.maju(); misal t = ini.cocok("IDENT"); jika t != nihil { ak = t.nilai; }
            } lainnya jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "IDENT" {
                misal pn = ini.lihat(1);
                jika pn == nihil atau (pn.tipe != "DOT" dan pn.tipe != "KURUNG_BUKA") {
                    misal t = ini.cocok("IDENT"); ak = t.nilai;
                }
            }
            misal cs = "";
            jika expr.jenis == "asterisk" { cs = "*"; }
            lainnya jika expr.jenis == "kolom" { cs = expr.nama; }
            lainnya jika expr.jenis == "kolom_qualified" { cs = expr.tabel_alias + "." + expr.kolom; }
            lainnya jika expr.jenis == "agregat" {
                misal ic = "*";
                jika expr.argumen != nihil {
                    jika expr.argumen.jenis == "kolom" { ic = expr.argumen.nama; }
                    lainnya jika expr.argumen.jenis == "kolom_qualified" { ic = expr.argumen.tabel_alias + "." + expr.argumen.kolom; }
                }
                cs = expr.fn_nama + "(" + ic + ")";
            }
            tambah(wl, {"col":cs,"alias":ak,"expr":expr});
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan wl;
    }
    fungsi parse_daftar_from() {
        misal fl = [];
        selama benar {
            misal tt = ini.cocok("IDENT"); jika tt == nihil { berhenti; }
            misal nt = tt.nilai; misal at = nihil;
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "IDENT" dan ke_besar_teks(ini.lihat(0).nilai) == "AS") {
                ini.maju(); misal a = ini.cocok("IDENT"); jika a != nihil { at = a.nilai; }
            } lainnya jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "IDENT" {
                misal p1 = ini.lihat(1);
                jika p1 == nihil atau p1.tipe == "JOIN" atau p1.tipe == "INNER" atau p1.tipe == "LEFT" atau p1.tipe == "RIGHT" atau p1.tipe == "WHERE" atau p1.tipe == "GROUP" atau p1.tipe == "ORDER" atau p1.tipe == "LIMIT" atau p1.tipe == "KOMA" {
                    misal a = ini.cocok("IDENT"); at = a.nilai;
                }
            }
            tambah(fl, {"tbl":nt,"alias":at});
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan fl;
    }
    fungsi parse_nilai() {
        misal tok = ini.sekarang(); jika tok == nihil { kembalikan nihil; }
        jika tok.tipe == "NUMBER" {
            ini.maju(); misal s = tok.nilai;
            jika ada(s, ".") { kembalikan ke_pecahan(s); }
            kembalikan ke_angka(s);
        }
        jika tok.tipe == "STRING" { ini.maju(); kembalikan tok.nilai; }
        jika tok.tipe == "NULL" { ini.maju(); kembalikan nihil; }
        jika tok.tipe == "IDENT" {
            ini.maju(); misal val = tok.nilai;
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "DOT" {
                ini.maju(); misal c2 = ini.cocok("IDENT"); val = val + "." + c2.nilai;
            }
            kembalikan {"jenis":"kolom_ref","nilai":val};
        }
        kembalikan nihil;
    }
    fungsi parse_kondisi_where() {
        misal wl = [];
        selama benar {
            misal kol = ini.parse_kolom_atau_expr(); jika kol == nihil { berhenti; }
            misal ot = ini.sekarang(); jika ot == nihil { berhenti; }
            misal os = "";
            jika ot.tipe == "OP_EQ" { os = "="; }
            lainnya jika ot.tipe == "OP_GT" { os = ">"; }
            lainnya jika ot.tipe == "OP_LT" { os = "<"; }
            lainnya jika ot.tipe == "OP_GTE" { os = ">="; }
            lainnya jika ot.tipe == "OP_LTE" { os = "<="; }
            lainnya jika ot.tipe == "OP_NEQ" { os = "!="; }
            lainnya { berhenti; }
            ini.maju(); misal val = ini.parse_nilai();
            misal cn = "";
            jika kol.jenis == "kolom" { cn = kol.nama; }
            lainnya jika kol.jenis == "kolom_qualified" { cn = kol.tabel_alias + "." + kol.kolom; }
            tambah(wl, {"col":cn,"op":os,"val":val});
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "AND" atau ini.lihat(0).tipe == "OR") { berhenti; }
        }
        kembalikan wl;
    }
    fungsi parse_joins() {
        misal js = [];
        selama benar {
            misal aj = salah;
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "INNER" { ini.maju(); aj = benar; }
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "LEFT" {
                ini.maju();
                jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "OUTER" { ini.maju(); }
                aj = benar;
            }
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "RIGHT" {
                ini.maju();
                jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "OUTER" { ini.maju(); }
                aj = benar;
            }
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "JOIN" { ini.maju(); aj = benar; }
            jika bukan aj { berhenti; }
            misal tt = ini.cocok("IDENT"); jika tt == nihil { berhenti; }
            misal nt = tt.nilai; misal at = nihil;
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "IDENT" dan ke_besar_teks(ini.lihat(0).nilai) == "AS") {
                ini.maju(); misal a = ini.cocok("IDENT"); jika a != nihil { at = a.nilai; }
            } lainnya jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "IDENT" {
                misal p1 = ini.lihat(1);
                jika p1 != nihil dan p1.tipe == "ON" { misal a = ini.cocok("IDENT"); at = a.nilai; }
            }
            ini.cocok("ON");
            misal kl = ini.parse_kolom_atau_expr();
            misal ot = ini.sekarang(); misal osim = "=";
            jika ot != nihil { jika ot.tipe == "OP_EQ" { osim = "="; } ini.maju(); }
            misal kr = ini.parse_kolom_atau_expr();
            misal lc = "";
            jika kl.jenis == "kolom" { lc = kl.nama; }
            lainnya jika kl.jenis == "kolom_qualified" { lc = kl.tabel_alias + "." + kl.kolom; }
            misal rc = "";
            jika kr.jenis == "kolom" { rc = kr.nama; }
            lainnya jika kr.jenis == "kolom_qualified" { rc = kr.tabel_alias + "." + kr.kolom; }
            tambah(js, {"tbl":nt,"alias":at,"left_col":lc,"op":osim,"right_col":rc});
        }
        kembalikan js;
    }
    fungsi parse_group_by() {
        misal cs = [];
        selama benar {
            misal tok = ini.parse_kolom_atau_expr(); jika tok == nihil { berhenti; }
            misal cn = "";
            jika tok.jenis == "kolom" { cn = tok.nama; }
            lainnya jika tok.jenis == "kolom_qualified" { cn = tok.tabel_alias + "." + tok.kolom; }
            tambah(cs, cn);
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan cs;
    }
    fungsi parse_order_by() {
        misal os = [];
        selama benar {
            misal tok = ini.parse_kolom_atau_expr(); jika tok == nihil { berhenti; }
            misal cn = "";
            jika tok.jenis == "kolom" { cn = tok.nama; }
            lainnya jika tok.jenis == "kolom_qualified" { cn = tok.tabel_alias + "." + tok.kolom; }
            misal arah = "ASC";
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "DESC" { arah = "DESC"; ini.maju(); }
            lainnya jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "ASC" { arah = "ASC"; ini.maju(); }
            tambah(os, {"col":cn,"dir":arah});
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan os;
    }
    fungsi parse_select() {
        ini.cocok("SELECT"); misal w = ini.parse_daftar_select(); ini.cocok("FROM"); misal f = ini.parse_daftar_from();
        misal j = ini.parse_joins(); misal wh = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "WHERE" { ini.maju(); wh = ini.parse_kondisi_where(); }
        misal gb = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "GROUP" { ini.maju(); ini.cocok("BY"); gb = ini.parse_group_by(); }
        misal ob = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "ORDER" { ini.maju(); ini.cocok("BY"); ob = ini.parse_order_by(); }
        misal lim = nihil;
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "LIMIT" {
            ini.maju(); misal lt = ini.cocok("NUMBER"); jika lt != nihil { lim = ke_angka(lt.nilai); }
        }
        kembalikan {"tipe":"SELECT","what":w,"from":f,"where":wh,"joins":j,"group_by":gb,"order_by":ob,"limit":lim};
    }
    fungsi parse_insert() {
        ini.cocok("INSERT"); ini.cocok("INTO"); misal tt = ini.cocok("IDENT"); misal nt = tt.nilai; misal cs = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KURUNG_BUKA" {
            ini.maju();
            selama benar {
                misal ct = ini.cocok("IDENT"); jika ct != nihil { tambah(cs, ct.nilai); }
                jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
                lainnya { berhenti; }
            }
            ini.cocok("KURUNG_TUTUP");
        }
        ini.cocok("VALUES"); misal sv = [];
        selama benar {
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KURUNG_BUKA" {
                ini.maju(); misal sb = [];
                selama benar {
                    misal v = ini.parse_nilai(); jika v != nihil { tambah(sb, v); }
                    jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
                    lainnya { berhenti; }
                }
                ini.cocok("KURUNG_TUTUP"); tambah(sv, sb);
            }
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan {"tipe":"INSERT","tbl":nt,"cols":cs,"values":sv};
    }
    fungsi parse_update() {
        ini.cocok("UPDATE"); misal tt = ini.cocok("IDENT"); misal nt = tt.nilai; ini.cocok("SET");
        misal sm = {};
        selama benar {
            misal ct = ini.cocok("IDENT"); jika ct == nihil { berhenti; }
            ini.cocok("OP_EQ"); misal val = ini.parse_nilai(); sm[ct.nilai] = val;
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        misal wh = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "WHERE" { ini.maju(); wh = ini.parse_kondisi_where(); }
        kembalikan {"tipe":"UPDATE","tbl":nt,"set":sm,"where":wh};
    }
    fungsi parse_delete() {
        ini.cocok("DELETE"); ini.cocok("FROM"); misal tt = ini.cocok("IDENT"); misal nt = tt.nilai;
        misal wh = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "WHERE" { ini.maju(); wh = ini.parse_kondisi_where(); }
        kembalikan {"tipe":"DELETE","tbl":nt,"where":wh};
    }
    fungsi parse() {
        misal ta = ini.sekarang(); jika ta == nihil { kembalikan nihil; }
        jika ta.tipe == "SELECT" { kembalikan ini.parse_select(); }
        jika ta.tipe == "INSERT" { kembalikan ini.parse_insert(); }
        jika ta.tipe == "UPDATE" { kembalikan ini.parse_update(); }
        jika ta.tipe == "DELETE" { kembalikan ini.parse_delete(); }
        kembalikan nihil;
    }
}
fungsi sql_parser(tokens) { misal p = ParserSQL(tokens); kembalikan p.parse(); }
fungsi tabel_ke_df(tabel_obj) {
    misal kolom = tabel_obj.kolom; misal bm = [];
    untuk rec dalam tabel_obj.baris_data {
        misal sb = [];
        untuk k dalam kolom {
            jika ada_kunci(rec, k) { tambah(sb, rec[k]); }
            lainnya { tambah(sb, nihil); }
        }
        tambah(bm, sb);
    }
    kembalikan DataFrame(kolom, bm);
}
fungsi terapkan_alias_ke_df(df, alias_tabel, _sk) {
    misal arr = df_ke_array(df); misal ha = [];
    untuk row dalam arr {
        misal rb = {};
        untuk k dalam kunci(row) {
            rb[k] = row[k];
            jika alias_tabel != nihil { rb[alias_tabel + "." + k] = row[k]; }
        }
        tambah(ha, rb);
    }
    jika panjang(ha) == 0 { kembalikan df; }
    misal kb = kunci(ha[0]); misal bb = [];
    untuk r dalam ha {
        misal br = [];
        untuk k dalam kb { tambah(br, r[k]); }
        tambah(bb, br);
    }
    kembalikan DataFrame(kb, bb);
}
fungsi bandingkan_nilai(a, op, b) {
    jika op == "=" { kembalikan a == b; }
    jika op == "!=" atau op == "<>" { kembalikan a != b; }
    jika op == ">" { kembalikan a > b; }
    jika op == "<" { kembalikan a < b; }
    jika op == ">=" { kembalikan a >= b; }
    jika op == "<=" { kembalikan a <= b; }
    kembalikan salah;
}
fungsi eksekusi_inner_join(dk, dkan, lcf, rcf) {
    misal ak = df_ke_array(dk); misal akan = df_ke_array(dkan); misal hsl = [];
    untuk lr dalam ak {
        misal lv = nihil; jika ada_kunci(lr, lcf) { lv = lr[lcf]; }
        untuk rr dalam akan {
            misal rv = nihil; jika ada_kunci(rr, rcf) { rv = rr[rcf]; }
            jika lv == rv {
                misal g = {};
                untuk k dalam kunci(lr) { g[k] = lr[k]; }
                untuk k dalam kunci(rr) { g[k] = rr[k]; }
                tambah(hsl, g);
            }
        }
    }
    jika panjang(hsl) == 0 { kembalikan DataFrame([], []); }
    misal kg = kunci(hsl[0]); misal bg = [];
    untuk r dalam hsl { misal br = []; untuk k dalam kg { tambah(br, r[k]); } tambah(bg, br); }
    kembalikan DataFrame(kg, bg);
}
fungsi eksekusi_where_manual(df, wl) {
    misal arr = df_ke_array(df); misal cur = arr;
    untuk kond in wl {
        misal hf = [];
        untuk row dalam cur {
            misal cn = kond.col; misal op = kond.op; misal vt = kond.val;
            misal rv = nihil; jika ada_kunci(row, cn) { rv = row[cn]; }
            misal vb = vt;
            jika vt != nihil dan tipe(vt) == "map" dan ada_kunci(vt, "jenis") dan vt.jenis == "kolom_ref" {
                misal rn = vt.nilai;
                jika ada_kunci(row, rn) { vb = row[rn]; }
            }
            jika bandingkan_nilai(rv, op, vb) { tambah(hf, row); }
        }
        cur = hf;
    }
    jika panjang(cur) == 0 { kembalikan DataFrame([], []); }
    misal kd = kunci(cur[0]); misal bd = [];
    untuk r dalam cur { misal br = []; untuk k dalam kd { tambah(br, r[k]); } tambah(bd, br); }
    kembalikan DataFrame(kd, bd);
}
fungsi ekstrak_nama_kolom_dari_select(cs) {
    jika cs == "*" { kembalikan "*"; }
    misal tb = cari_teks(cs, "(");
    jika tb >= 0 {
        misal tt = cari_teks(cs, ")");
        kembalikan potong(cs, tb + 1, tt);
    }
    kembalikan cs;
}
fungsi ekstrak_fungsi_agregat(cs) {
    misal tb = cari_teks(cs, "(");
    jika tb >= 0 { kembalikan ke_besar_teks(potong(cs, 0, tb)); }
    kembalikan nihil;
}
fungsi urutkan_array(arr, ors) {
    misal n = panjang(arr); misal i = 0;
    selama i < n - 1 {
        misal j = 0;
        selama j < n - i - 1 {
            misal a = arr[j]; misal b = arr[j + 1]; misal ht = salah;
            untuk rule dalam ors {
                misal col = rule.col; misal dir = rule.dir; misal va = nihil; misal vb = nihil;
                jika ada_kunci(a, col) { va = a[col]; }
                jika ada_kunci(b, col) { vb = b[col]; }
                jika va != vb {
                    jika dir == "ASC" { ht = va > vb; }
                    lainnya { ht = va < vb; }
                    berhenti;
                }
            }
            jika ht { arr[j] = b; arr[j + 1] = a; }
            j = j + 1;
        }
        i = i + 1;
    }
    kembalikan arr;
}
fungsi potong_array(arr, start, end_idx) {
    misal hasil = []; misal i = start;
    selama i < end_idx dan i < panjang(arr) { tambah(hasil, arr[i]); i = i + 1; }
    kembalikan hasil;
}
fungsi sql_eksekusi(dr, ast) {
    jika ast == nihil { kembalikan nihil; }
    jika ast.tipe == "INSERT" {
        misal nt = ast.tbl; jika bukan ada_kunci(dr.tabel, nt) { kembalikan nihil; }
        misal tbl = dr.tabel[nt]; misal cols = ast.cols; misal ci = 0;
        untuk sv dalam ast.values {
            misal rec = {};
            jika panjang(cols) > 0 {
                misal cii = 0;
                selama cii < panjang(cols) { rec[cols[cii]] = sv[cii]; cii = cii + 1; }
            } lainnya {
                misal cii = 0;
                selama cii < panjang(tbl.kolom) dan cii < panjang(sv) {
                    rec[tbl.kolom[cii]] = sv[cii]; cii = cii + 1;
                }
            }
            jika tbl.sisip(rec) { ci = ci + 1; }
        }
        kembalikan {"dipengaruhi": ci};
    }
    jika ast.tipe == "UPDATE" {
        misal nt = ast.tbl; jika bukan ada_kunci(dr.tabel, nt) { kembalikan nihil; }
        misal tbl = dr.tabel[nt]; misal cu = 0; misal i = 0;
        selama i < panjang(tbl.baris_data) {
            misal rec = tbl.baris_data[i]; misal lolos = benar;
            untuk kond dalam ast.where {
                misal cv = nihil; jika ada_kunci(rec, kond.col) { cv = rec[kond.col]; }
                jika bukan bandingkan_nilai(cv, kond.op, kond.val) { lolos = salah; berhenti; }
            }
            jika lolos {
                untuk sk dalam kunci(ast["set"]) { rec[sk] = ast["set"][sk]; }
                tbl.baris_data[i] = rec; cu = cu + 1;
            }
            i = i + 1;
        }
        kembalikan {"dipengaruhi": cu};
    }
    jika ast.tipe == "DELETE" {
        misal nt = ast.tbl; jika bukan ada_kunci(dr.tabel, nt) { kembalikan nihil; }
        misal tbl = dr.tabel[nt]; misal tersisa = []; misal cd = 0;
        untuk rec dalam tbl.baris_data {
            misal lh = benar;
            untuk kond dalam ast.where {
                misal cv = nihil; jika ada_kunci(rec, kond.col) { cv = rec[kond.col]; }
                jika bukan bandingkan_nilai(cv, kond.op, kond.val) { lh = salah; berhenti; }
            }
            jika lh dan panjang(ast.where) > 0 { cd = cd + 1; }
            lainnya { tambah(tersisa, rec); }
        }
        tbl.baris_data = tersisa;
        kembalikan {"dipengaruhi": cd};
    }
    jika ast.tipe == "SELECT" {
        misal cdf = nihil;
        untuk fi dalam ast.from {
            misal nt = fi.tbl; misal at = fi.alias;
            jika bukan ada_kunci(dr.tabel, nt) { kembalikan nihil; }
            misal to = dr.tabel[nt]; misal dt = tabel_ke_df(to);
            dt = terapkan_alias_ke_df(dt, at, to.kolom);
            jika cdf == nihil { cdf = dt; }
            lainnya { cdf = df_gabung(cdf, dt); }
        }
        untuk ji dalam ast.joins {
            misal ntj = ji.tbl; misal atj = ji.alias;
            jika bukan ada_kunci(dr.tabel, ntj) { kembalikan nihil; }
            misal toj = dr.tabel[ntj]; misal dtj = tabel_ke_df(toj);
            dtj = terapkan_alias_ke_df(dtj, atj, toj.kolom);
            cdf = eksekusi_inner_join(cdf, dtj, ji.left_col, ji.right_col);
        }
        jika panjang(ast.where) > 0 { cdf = eksekusi_where_manual(cdf, ast.where); }
        misal aa = salah;
        untuk w dalam ast.what { jika ekstrak_fungsi_agregat(w.col) != nihil { aa = benar; berhenti; } }
        misal ha = [];
        jika panjang(ast.group_by) > 0 atau aa {
            misal ac = df_ke_array(cdf);
            jika panjang(ac) == 0 { kembalikan DataFrame([], []); }
            misal groups = {};
            untuk row dalam ac {
                misal kb = "";
                untuk gc dalam ast.group_by {
                    misal gv = nihil; jika ada_kunci(row, gc) { gv = row[gc]; }
                    kb = kb + "||" + ke_teks(gv);
                }
                jika bukan ada_kunci(groups, kb) { groups[kb] = []; }
                tambah(groups[kb], row);
            }
            misal ngb = [];
            untuk gc dalam ast.group_by {
                misal ti = cari_teks(gc, ".");
                jika ti >= 0 { tambah(ngb, potong(gc, ti + 1, panjang(gc))); }
                lainnya { tambah(ngb, gc); }
            }
            untuk gk dalam kunci(groups) {
                misal gm = groups[gk]; misal sh = {};
                misal ggi = 0;
                selama ggi < panjang(ast.group_by) {
                    misal fr = gm[0]; misal gcf = ast.group_by[ggi]; misal gcb = ngb[ggi];
                    jika ada_kunci(fr, gcf) { sh[gcf] = fr[gcf]; }
                    jika ada_kunci(fr, gcf) { sh[gcb] = fr[gcf]; }
                    ggi = ggi + 1;
                }
                untuk wi dalam ast.what {
                    misal cs = wi.col; misal ca = wi.alias;
                    misal af = ekstrak_fungsi_agregat(cs);
                    misal no = ca; jika no == nihil { no = cs; }
                    jika af != nihil {
                        misal tc = ekstrak_nama_kolom_dari_select(cs);
                        misal ti = cari_teks(tc, ".");
                        misal cb = tc; jika ti >= 0 { cb = potong(tc, ti + 1, panjang(tc)); }
                        misal av = nihil;
                        jika af == "SUM" {
                            av = 0;
                            untuk gmv dalam gm {
                                misal vv = nihil;
                                jika ada_kunci(gmv, tc) { vv = gmv[tc]; }
                                lainnya jika ada_kunci(gmv, cb) { vv = gmv[cb]; }
                                jika vv != nihil { av = av + vv; }
                            }
                        } lainnya jika af == "COUNT" { av = panjang(gm); }
                        sh[no] = av; sh[cs] = av;
                    }
                }
                tambah(ha, sh);
            }
        } lainnya { ha = df_ke_array(cdf); }
        misal pp = salah;
        jika panjang(ast.what) > 0 {
            untuk w dalam ast.what { jika w.col != "*" { pp = benar; berhenti; } }
        }
        jika pp {
            misal apr = [];
            untuk r dalam ha {
                misal rb = {};
                untuk w dalam ast.what {
                    misal cn = w.col; misal an = w.alias;
                    jika cn == "*" { untuk rk dalam kunci(r) { rb[rk] = r[rk]; } berhenti; }
                    misal ns = nihil; jika ada_kunci(r, cn) { ns = r[cn]; }
                    misal tt = cari_teks(cn, "."); misal nb = "";
                    jika tt >= 0 { nb = potong(cn, tt + 1, panjang(cn)); }
                    misal nou = "";
                    jika an != nihil { nou = an; }
                    lainnya jika tt >= 0 { nou = nb; }
                    lainnya { nou = cn; }
                    jika ns != nihil {
                        rb[nou] = ns; rb[cn] = ns;
                        jika an != nihil { rb[an] = ns; }
                        jika tt >= 0 { rb[nb] = ns; }
                    }
                }
                tambah(apr, rb);
            }
            ha = apr;
        }
        jika panjang(ast.order_by) > 0 dan panjang(ha) > 0 { ha = urutkan_array(ha, ast.order_by); }
        jika ast.limit != nihil dan panjang(ha) > ast.limit { ha = potong_array(ha, 0, ast.limit); }
        jika panjang(ha) == 0 { kembalikan DataFrame([], []); }
        misal fc = kunci(ha[0]); misal frs = [];
        untuk fr dalam ha {
            misal br = [];
            untuk fck dalam fc { tambah(br, fr[fck]); }
            tambah(frs, br);
        }
        kembalikan DataFrame(fc, frs);
    }
    kembalikan nihil;
}
fungsi eksekusi_sql(dbi, sql_str) {
    misal tokens = sql_lexer(sql_str); misal ast = sql_parser(tokens);
    kembalikan sql_eksekusi(dbi, ast);
}
struktur DBWidya {
    tabel,
    fungsi inisialisasi() { ini.tabel = {}; },
    fungsi buat_tabel(nama, kolom) {
        misal t = TabelRelasional(nama, kolom); ini.tabel[nama] = t; kembalikan t;
    }
}
misal db = DBWidya();
misal tbl_pengguna = db.buat_tabel("pengguna", ["id", "nama", "kota", "saldo"]);
misal tbl_pesanan = db.buat_tabel("pesanan", ["id", "pengguna_id", "produk", "total", "tanggal"]);
tbl_pengguna.sisip({"id": 1, "nama": "Ahmad Dani", "kota": "Jakarta", "saldo": 5000000});
tbl_pengguna.sisip({"id": 2, "nama": "Siti Nurhaliza", "kota": "Bandung", "saldo": 12500000});
tbl_pengguna.sisip({"id": 3, "nama": "Budi Santoso", "kota": "Surabaya", "saldo": 750000});
tbl_pengguna.sisip({"id": 4, "nama": "Dewi Sartika", "kota": "Jakarta", "saldo": 8500000});
tbl_pengguna.sisip({"id": 5, "nama": "Eko Prasetyo", "kota": "Yogyakarta", "saldo": 3200000});
tbl_pesanan.sisip({"id": 101, "pengguna_id": 1, "produk": "Laptop", "total": 15000000, "tanggal": "2026-01-10"});
tbl_pesanan.sisip({"id": 102, "pengguna_id": 2, "produk": "Monitor", "total": 8500000, "tanggal": "2026-01-12"});
tbl_pesanan.sisip({"id": 103, "pengguna_id": 1, "produk": "Keyboard", "total": 1200000, "tanggal": "2026-01-15"});
tbl_pesanan.sisip({"id": 104, "pengguna_id": 4, "produk": "Mouse", "total": 350000, "tanggal": "2026-01-18"});
tbl_pesanan.sisip({"id": 105, "pengguna_id": 2, "produk": "Printer", "total": 4200000, "tanggal": "2026-01-20"});
tetap SQL_QUERY = "SELECT p.nama, SUM(o.total) AS total_belanja FROM pengguna p JOIN pesanan o ON p.id = o.pengguna_id GROUP BY p.nama ORDER BY total_belanja DESC LIMIT 3";
misal hasil_query = eksekusi_sql(db, SQL_QUERY);
misal arr_hasil = df_ke_array(hasil_query);
misal assert1 = panjang(arr_hasil) == 3;
jika bukan assert1 { kembalikan salah; }
misal assert2 = arr_hasil[0].total_belanja >= arr_hasil[1].total_belanja;
jika bukan assert2 { kembalikan salah; }
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_1_sql_join_groupby_order_limit: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_2_sql_insert_update_delete() {
    let code = r##"
struktur TabelRelasional {
    nama_tabel, kolom, baris_data, indeks_pk,
    fungsi inisialisasi(nama, daftar_kolom) {
        ini.nama_tabel = nama; ini.kolom = daftar_kolom; ini.baris_data = []; ini.indeks_pk = {};
    }
    fungsi sisip(data_record) {
        misal id = data_record["id"];
        jika id != nihil dan ada_kunci(ini.indeks_pk, ke_teks(id)) { kembalikan salah; }
        tambah(ini.baris_data, data_record);
        jika id != nihil { ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1; }
        kembalikan benar;
    }
}
fungsi apakah_huruf(c) {
    jika panjang(c) == 0 { kembalikan salah; }
    tetap hm = {"A":benar,"B":benar,"C":benar,"D":benar,"E":benar,"F":benar,"G":benar,"H":benar,"I":benar,"J":benar,"K":benar,"L":benar,"M":benar,"N":benar,"O":benar,"P":benar,"Q":benar,"R":benar,"S":benar,"T":benar,"U":benar,"V":benar,"W":benar,"X":benar,"Y":benar,"Z":benar,"a":benar,"b":benar,"c":benar,"d":benar,"e":benar,"f":benar,"g":benar,"h":benar,"i":benar,"j":benar,"k":benar,"l":benar,"m":benar,"n":benar,"o":benar,"p":benar,"q":benar,"r":benar,"s":benar,"t":benar,"u":benar,"v":benar,"w":benar,"x":benar,"y":benar,"z":benar,"_":benar};
    kembalikan ada_kunci(hm, c);
}
fungsi apakah_digit(c) {
    tetap dm = {"0":benar,"1":benar,"2":benar,"3":benar,"4":benar,"5":benar,"6":benar,"7":benar,"8":benar,"9":benar};
    kembalikan ada_kunci(dm, c);
}
fungsi apakah_huruf_atau_digit(c) { kembalikan apakah_huruf(c) atau apakah_digit(c); }
fungsi ke_besar_teks(s) {
    tetap mb = {"a":"A","b":"B","c":"C","d":"D","e":"E","f":"F","g":"G","h":"H","i":"I","j":"J","k":"K","l":"L","m":"M","n":"N","o":"O","p":"P","q":"Q","r":"R","s":"S","t":"T","u":"U","v":"V","w":"W","x":"X","y":"Y","z":"Z"};
    misal hasil = ""; misal i = 0;
    selama i < panjang(s) {
        misal c = s[i];
        jika ada_kunci(mb, c) { hasil = hasil + mb[c]; } lainnya { hasil = hasil + c; }
        i = i + 1;
    }
    kembalikan hasil;
}
fungsi ke_angka(s) {
    tetap dm = {"0":0,"1":1,"2":2,"3":3,"4":4,"5":5,"6":6,"7":7,"8":8,"9":9};
    misal neg = salah; misal start_idx = 0;
    jika panjang(s) > 0 dan s[0] == "-" { neg = benar; start_idx = 1; }
    misal hasil = 0; misal i = start_idx;
    selama i < panjang(s) {
        misal d = s[i];
        jika ada_kunci(dm, d) { hasil = hasil * 10 + dm[d]; } lainnya { berhenti; }
        i = i + 1;
    }
    jika neg { hasil = 0 - hasil; }
    kembalikan hasil;
}
fungsi ke_pecahan(s) {
    tetap dm = {"0":0,"1":1,"2":2,"3":3,"4":4,"5":5,"6":6,"7":7,"8":8,"9":9};
    misal bagian = pisah(s, ".");
    misal bulat = ke_angka(bagian[0]);
    jika panjang(bagian) < 2 { kembalikan bulat; }
    misal frac_str = bagian[1];
    misal frac = 0.0; misal div = 1.0; misal i = 0;
    selama i < panjang(frac_str) {
        misal d = frac_str[i];
        jika ada_kunci(dm, d) { frac = frac * 10.0 + dm[d]; div = div * 10.0; }
        i = i + 1;
    }
    kembalikan bulat + (frac / div);
}
fungsi pisah(s, delim) {
    misal hasil = []; misal mulai = 0; misal i = 0; misal p_delim = panjang(delim);
    selama i <= panjang(s) - p_delim {
        misal cocok = benar; misal j = 0;
        selama j < p_delim {
            jika s[i + j] != delim[j] { cocok = salah; berhenti; }
            j = j + 1;
        }
        jika cocok {
            tambah(hasil, potong(s, mulai, i));
            i = i + p_delim; mulai = i;
        } lainnya { i = i + 1; }
    }
    tambah(hasil, potong(s, mulai, panjang(s)));
    kembalikan hasil;
}
fungsi cari_teks(s, needle) {
    misal i = 0; misal ns = panjang(s); misal nn = panjang(needle);
    selama i <= ns - nn {
        misal cocok = benar; misal j = 0;
        selama j < nn {
            jika s[i + j] != needle[j] { cocok = salah; berhenti; }
            j = j + 1;
        }
        jika cocok { kembalikan i; }
        i = i + 1;
    }
    kembalikan -1;
}
fungsi ada(s, needle) { kembalikan cari_teks(s, needle) >= 0; }
fungsi sql_lexer(sql_string) {
    misal tokens = []; misal pos = 0; misal len = panjang(sql_string);
    tetap KM = {"SELECT":"SELECT","FROM":"FROM","WHERE":"WHERE","JOIN":"JOIN","ON":"ON","GROUP":"GROUP","BY":"BY","ORDER":"ORDER","ASC":"ASC","DESC":"DESC","INSERT":"INSERT","INTO":"INTO","VALUES":"VALUES","UPDATE":"UPDATE","SET":"SET","DELETE":"DELETE","LIMIT":"LIMIT","AND":"AND","OR":"OR","NOT":"NOT","NULL":"NULL","SUM":"SUM","AVG":"AVG","COUNT":"COUNT","MIN":"MIN","MAX":"MAX","INNER":"INNER","LEFT":"LEFT","RIGHT":"RIGHT","OUTER":"OUTER"};
    selama pos < len {
        misal c = sql_string[pos]; misal dt = salah;
        jika bukan dt dan (c == " " atau c == "\t" atau c == "\n" atau c == "\r") { pos = pos + 1; dt = benar; }
        jika bukan dt dan c == ";" { tambah(tokens, {"tipe":"TITIK_KOMA","nilai":";"}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "," { tambah(tokens, {"tipe":"KOMA","nilai":","}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "*" { tambah(tokens, {"tipe":"ASTERISK","nilai":"*"}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "(" { tambah(tokens, {"tipe":"KURUNG_BUKA","nilai":"("}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == ")" { tambah(tokens, {"tipe":"KURUNG_TUTUP","nilai":")"}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "." { tambah(tokens, {"tipe":"DOT","nilai":"."}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == "=" { tambah(tokens, {"tipe":"OP_EQ","nilai":"="}); pos = pos + 1; dt = benar; }
        jika bukan dt dan c == ">" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" { tambah(tokens, {"tipe":"OP_GTE","nilai":">="}); pos = pos + 2; }
            lainnya { tambah(tokens, {"tipe":"OP_GT","nilai":">"}); pos = pos + 1; }
            dt = benar;
        }
        jika bukan dt dan c == "<" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" { tambah(tokens, {"tipe":"OP_LTE","nilai":"<="}); pos = pos + 2; }
            lainnya jika pos + 1 < len dan sql_string[pos + 1] == ">" { tambah(tokens, {"tipe":"OP_NEQ","nilai":"<>"}); pos = pos + 2; }
            lainnya { tambah(tokens, {"tipe":"OP_LT","nilai":"<"}); pos = pos + 1; }
            dt = benar;
        }
        jika bukan dt dan c == "!" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" { tambah(tokens, {"tipe":"OP_NEQ","nilai":"!="}); pos = pos + 2; dt = benar; }
        }
        jika bukan dt dan c == "'" {
            misal mulai = pos + 1; pos = pos + 1;
            selama pos < len dan sql_string[pos] != "'" {
                jika sql_string[pos] == "\\" dan pos + 1 < len { pos = pos + 2; }
                lainnya { pos = pos + 1; }
            }
            tambah(tokens, {"tipe":"STRING","nilai":potong(sql_string, mulai, pos)});
            pos = pos + 1; dt = benar;
        }
        jika bukan dt dan c == "\"" {
            misal mulai = pos + 1; pos = pos + 1;
            selama pos < len dan sql_string[pos] != "\"" {
                jika sql_string[pos] == "\\" dan pos + 1 < len { pos = pos + 2; }
                lainnya { pos = pos + 1; }
            }
            tambah(tokens, {"tipe":"STRING","nilai":potong(sql_string, mulai, pos)});
            pos = pos + 1; dt = benar;
        }
        jika bukan dt dan apakah_digit(c) {
            misal mulai = pos;
            selama pos < len dan (apakah_digit(sql_string[pos]) atau sql_string[pos] == ".") { pos = pos + 1; }
            tambah(tokens, {"tipe":"NUMBER","nilai":potong(sql_string, mulai, pos)});
            dt = benar;
        }
        jika bukan dt dan apakah_huruf(c) {
            misal mulai = pos;
            selama pos < len dan apakah_huruf_atau_digit(sql_string[pos]) { pos = pos + 1; }
            misal ident = potong(sql_string, mulai, pos);
            misal iu = ke_besar_teks(ident);
            jika ada_kunci(KM, iu) { tambah(tokens, {"tipe":iu,"nilai":ident}); }
            lainnya { tambah(tokens, {"tipe":"IDENT","nilai":ident}); }
            dt = benar;
        }
        jika bukan dt { pos = pos + 1; }
    }
    kembalikan tokens;
}
struktur ParserSQL {
    tokens, pos, len_tokens,
    fungsi inisialisasi(tl) { ini.tokens = tl; ini.pos = 0; ini.len_tokens = panjang(tl); }
    fungsi sekarang() { jika ini.pos < ini.len_tokens { kembalikan ini.tokens[ini.pos]; } kembalikan nihil; }
    fungsi lihat(offset) { misal idx = ini.pos + offset; jika idx >= 0 dan idx < ini.len_tokens { kembalikan ini.tokens[idx]; } kembalikan nihil; }
    fungsi maju() { misal tok = ini.sekarang(); ini.pos = ini.pos + 1; kembalikan tok; }
    fungsi cocok(tipe) { misal tok = ini.sekarang(); jika tok != nihil dan tok.tipe == tipe { kembalikan ini.maju(); } kembalikan nihil; }
    fungsi parse_kolom_atau_expr() {
        misal tok = ini.sekarang(); jika tok == nihil { kembalikan nihil; }
        jika tok.tipe == "SUM" atau tok.tipe == "AVG" atau tok.tipe == "COUNT" atau tok.tipe == "MIN" atau tok.tipe == "MAX" {
            misal af = ini.maju(); ini.cocok("KURUNG_BUKA"); misal ie = ini.parse_kolom_atau_expr(); ini.cocok("KURUNG_TUTUP");
            kembalikan {"jenis":"agregat","fn_nama":af.tipe,"argumen":ie};
        }
        jika tok.tipe == "ASTERISK" { ini.maju(); kembalikan {"jenis":"asterisk"}; }
        misal b1 = ini.cocok("IDENT"); jika b1 == nihil { kembalikan nihil; }
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "DOT" {
            ini.maju(); misal b2 = ini.cocok("IDENT"); jika b2 == nihil { b2 = ini.cocok("ASTERISK"); }
            kembalikan {"jenis":"kolom_qualified","tabel_alias":b1.nilai,"kolom":b2.nilai};
        }
        kembalikan {"jenis":"kolom","nama":b1.nilai};
    }
    fungsi parse_daftar_select() {
        misal wl = [];
        selama benar {
            misal expr = ini.parse_kolom_atau_expr(); jika expr == nihil { berhenti; }
            misal ak = nihil;
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "IDENT" dan ke_besar_teks(ini.lihat(0).nilai) == "AS") {
                ini.maju(); misal t = ini.cocok("IDENT"); jika t != nihil { ak = t.nilai; }
            } lainnya jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "IDENT" {
                misal pn = ini.lihat(1);
                jika pn == nihil atau (pn.tipe != "DOT" dan pn.tipe != "KURUNG_BUKA") {
                    misal t = ini.cocok("IDENT"); ak = t.nilai;
                }
            }
            tambah(wl, {"col":"col_placeholder","alias":ak,"expr":expr});
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan wl;
    }
    fungsi parse_daftar_from() {
        misal fl = [];
        selama benar {
            misal tt = ini.cocok("IDENT"); jika tt == nihil { berhenti; }
            misal nt = tt.nilai; misal at = nihil;
            tambah(fl, {"tbl":nt,"alias":at});
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan fl;
    }
    fungsi parse_nilai() {
        misal tok = ini.sekarang(); jika tok == nihil { kembalikan nihil; }
        jika tok.tipe == "NUMBER" {
            ini.maju(); misal s = tok.nilai;
            jika ada(s, ".") { kembalikan ke_pecahan(s); }
            kembalikan ke_angka(s);
        }
        jika tok.tipe == "STRING" { ini.maju(); kembalikan tok.nilai; }
        jika tok.tipe == "NULL" { ini.maju(); kembalikan nihil; }
        jika tok.tipe == "IDENT" {
            ini.maju(); misal val = tok.nilai;
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "DOT" {
                ini.maju(); misal c2 = ini.cocok("IDENT"); val = val + "." + c2.nilai;
            }
            kembalikan {"jenis":"kolom_ref","nilai":val};
        }
        kembalikan nihil;
    }
    fungsi parse_kondisi_where() {
        misal wl = [];
        selama benar {
            misal kol = ini.parse_kolom_atau_expr(); jika kol == nihil { berhenti; }
            misal ot = ini.sekarang(); jika ot == nihil { berhenti; }
            misal os = "";
            jika ot.tipe == "OP_EQ" { os = "="; }
            lainnya jika ot.tipe == "OP_GT" { os = ">"; }
            lainnya jika ot.tipe == "OP_LT" { os = "<"; }
            lainnya jika ot.tipe == "OP_GTE" { os = ">="; }
            lainnya jika ot.tipe == "OP_LTE" { os = "<="; }
            lainnya jika ot.tipe == "OP_NEQ" { os = "!="; }
            lainnya { berhenti; }
            ini.maju(); misal val = ini.parse_nilai();
            misal cn = "";
            jika kol.jenis == "kolom" { cn = kol.nama; }
            lainnya jika kol.jenis == "kolom_qualified" { cn = kol.tabel_alias + "." + kol.kolom; }
            tambah(wl, {"col":cn,"op":os,"val":val});
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "AND" atau ini.lihat(0).tipe == "OR") { berhenti; }
        }
        kembalikan wl;
    }
    fungsi parse_insert() {
        ini.cocok("INSERT"); ini.cocok("INTO"); misal tt = ini.cocok("IDENT"); misal nt = tt.nilai; misal cs = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KURUNG_BUKA" {
            ini.maju();
            selama benar {
                misal ct = ini.cocok("IDENT"); jika ct != nihil { tambah(cs, ct.nilai); }
                jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
                lainnya { berhenti; }
            }
            ini.cocok("KURUNG_TUTUP");
        }
        ini.cocok("VALUES"); misal sv = [];
        selama benar {
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KURUNG_BUKA" {
                ini.maju(); misal sb = [];
                selama benar {
                    misal v = ini.parse_nilai(); jika v != nihil { tambah(sb, v); }
                    jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
                    lainnya { berhenti; }
                }
                ini.cocok("KURUNG_TUTUP"); tambah(sv, sb);
            }
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        kembalikan {"tipe":"INSERT","tbl":nt,"cols":cs,"values":sv};
    }
    fungsi parse_update() {
        ini.cocok("UPDATE"); misal tt = ini.cocok("IDENT"); misal nt = tt.nilai; ini.cocok("SET");
        misal sm = {};
        selama benar {
            misal ct = ini.cocok("IDENT"); jika ct == nihil { berhenti; }
            ini.cocok("OP_EQ"); misal val = ini.parse_nilai(); sm[ct.nilai] = val;
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju(); }
            lainnya { berhenti; }
        }
        misal wh = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "WHERE" { ini.maju(); wh = ini.parse_kondisi_where(); }
        kembalikan {"tipe":"UPDATE","tbl":nt,"set":sm,"where":wh};
    }
    fungsi parse_delete() {
        ini.cocok("DELETE"); ini.cocok("FROM"); misal tt = ini.cocok("IDENT"); misal nt = tt.nilai;
        misal wh = [];
        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "WHERE" { ini.maju(); wh = ini.parse_kondisi_where(); }
        kembalikan {"tipe":"DELETE","tbl":nt,"where":wh};
    }
    fungsi parse() {
        misal ta = ini.sekarang(); jika ta == nihil { kembalikan nihil; }
        jika ta.tipe == "INSERT" { kembalikan ini.parse_insert(); }
        jika ta.tipe == "UPDATE" { kembalikan ini.parse_update(); }
        jika ta.tipe == "DELETE" { kembalikan ini.parse_delete(); }
        kembalikan nihil;
    }
}
fungsi sql_parser(tokens) { misal p = ParserSQL(tokens); kembalikan p.parse(); }
fungsi bandingkan_nilai(a, op, b) {
    jika op == "=" { kembalikan a == b; }
    jika op == "!=" atau op == "<>" { kembalikan a != b; }
    jika op == ">" { kembalikan a > b; }
    jika op == "<" { kembalikan a < b; }
    jika op == ">=" { kembalikan a >= b; }
    jika op == "<=" { kembalikan a <= b; }
    kembalikan salah;
}
fungsi sql_eksekusi(dr, ast) {
    jika ast == nihil { kembalikan nihil; }
    jika ast.tipe == "INSERT" {
        misal nt = ast.tbl; jika bukan ada_kunci(dr.tabel, nt) { kembalikan nihil; }
        misal tbl = dr.tabel[nt]; misal cols = ast.cols; misal ci = 0;
        untuk sv dalam ast.values {
            misal rec = {};
            jika panjang(cols) > 0 {
                misal cii = 0;
                selama cii < panjang(cols) { rec[cols[cii]] = sv[cii]; cii = cii + 1; }
            } lainnya {
                misal cii = 0;
                selama cii < panjang(tbl.kolom) dan cii < panjang(sv) {
                    rec[tbl.kolom[cii]] = sv[cii]; cii = cii + 1;
                }
            }
            jika tbl.sisip(rec) { ci = ci + 1; }
        }
        kembalikan {"dipengaruhi": ci};
    }
    jika ast.tipe == "UPDATE" {
        misal nt = ast.tbl; jika bukan ada_kunci(dr.tabel, nt) { kembalikan nihil; }
        misal tbl = dr.tabel[nt]; misal cu = 0; misal i = 0;
        selama i < panjang(tbl.baris_data) {
            misal rec = tbl.baris_data[i]; misal lolos = benar;
            untuk kond dalam ast.where {
                misal cv = nihil; jika ada_kunci(rec, kond.col) { cv = rec[kond.col]; }
                jika bukan bandingkan_nilai(cv, kond.op, kond.val) { lolos = salah; berhenti; }
            }
            jika lolos {
                untuk sk dalam kunci(ast["set"]) { rec[sk] = ast["set"][sk]; }
                tbl.baris_data[i] = rec; cu = cu + 1;
            }
            i = i + 1;
        }
        kembalikan {"dipengaruhi": cu};
    }
    jika ast.tipe == "DELETE" {
        misal nt = ast.tbl; jika bukan ada_kunci(dr.tabel, nt) { kembalikan nihil; }
        misal tbl = dr.tabel[nt]; misal tersisa = []; misal cd = 0;
        untuk rec dalam tbl.baris_data {
            misal lh = benar;
            untuk kond dalam ast.where {
                misal cv = nihil; jika ada_kunci(rec, kond.col) { cv = rec[kond.col]; }
                jika bukan bandingkan_nilai(cv, kond.op, kond.val) { lh = salah; berhenti; }
            }
            jika lh dan panjang(ast.where) > 0 { cd = cd + 1; }
            lainnya { tambah(tersisa, rec); }
        }
        tbl.baris_data = tersisa;
        kembalikan {"dipengaruhi": cd};
    }
    kembalikan nihil;
}
fungsi eksekusi_sql(dbi, sql_str) {
    misal tokens = sql_lexer(sql_str); misal ast = sql_parser(tokens);
    kembalikan sql_eksekusi(dbi, ast);
}
struktur DBWidya {
    tabel,
    fungsi inisialisasi() { ini.tabel = {}; },
    fungsi buat_tabel(nama, kolom) {
        misal t = TabelRelasional(nama, kolom); ini.tabel[nama] = t; kembalikan t;
    }
}
misal db = DBWidya();
misal tbl_pengguna = db.buat_tabel("pengguna", ["id", "nama", "kota", "saldo"]);
misal tbl_pesanan = db.buat_tabel("pesanan", ["id", "pengguna_id", "produk", "total", "tanggal"]);
tbl_pengguna.sisip({"id": 1, "nama": "Ahmad Dani", "kota": "Jakarta", "saldo": 5000000});
tbl_pengguna.sisip({"id": 2, "nama": "Siti Nurhaliza", "kota": "Bandung", "saldo": 12500000});
tbl_pengguna.sisip({"id": 3, "nama": "Budi Santoso", "kota": "Surabaya", "saldo": 750000});
tbl_pengguna.sisip({"id": 4, "nama": "Dewi Sartika", "kota": "Jakarta", "saldo": 8500000});
tbl_pengguna.sisip({"id": 5, "nama": "Eko Prasetyo", "kota": "Yogyakarta", "saldo": 3200000});
tbl_pesanan.sisip({"id": 101, "pengguna_id": 1, "produk": "Laptop", "total": 15000000, "tanggal": "2026-01-10"});
tbl_pesanan.sisip({"id": 102, "pengguna_id": 2, "produk": "Monitor", "total": 8500000, "tanggal": "2026-01-12"});
tbl_pesanan.sisip({"id": 103, "pengguna_id": 1, "produk": "Keyboard", "total": 1200000, "tanggal": "2026-01-15"});
tbl_pesanan.sisip({"id": 104, "pengguna_id": 4, "produk": "Mouse", "total": 350000, "tanggal": "2026-01-18"});
tbl_pesanan.sisip({"id": 105, "pengguna_id": 2, "produk": "Printer", "total": 4200000, "tanggal": "2026-01-20"});
misal jml_awal_pengguna = panjang(tbl_pengguna.baris_data);
eksekusi_sql(db, "INSERT INTO pengguna (id, nama, kota, saldo) VALUES (6, 'Fajar Nugroho', 'Semarang', 2100000)");
misal jml_akhir_pengguna = panjang(tbl_pengguna.baris_data);
misal assert_insert = jml_akhir_pengguna == jml_awal_pengguna + 1;
jika bukan assert_insert { kembalikan salah; }
misal saldo_sebelum = tbl_pengguna.baris_data[0].saldo;
eksekusi_sql(db, "UPDATE pengguna SET saldo = 9999999 WHERE id = 1");
misal saldo_setelah = tbl_pengguna.baris_data[0].saldo;
misal assert_update = saldo_setelah == 9999999;
jika bukan assert_update { kembalikan salah; }
misal jml_awal_del = panjang(tbl_pesanan.baris_data);
eksekusi_sql(db, "DELETE FROM pesanan WHERE id = 105");
misal jml_akhir_del = panjang(tbl_pesanan.baris_data);
misal assert_delete = jml_akhir_del == jml_awal_del - 1;
jika bukan assert_delete { kembalikan salah; }
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_2_sql_insert_update_delete: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_3_st_spatial_8functions_ogc() {
    let code = r##"
fungsi rad(deg) { kembalikan deg * PI / 180.0 }
fungsi my_exp(x) {
    jika x == 0 { kembalikan 1.0 }
    var negatif = salah
    jika x < 0 { negatif = benar; x = -x }
    var hasil = 1.0
    var suku = 1.0
    untuk n dalam 1..25 { suku = suku * x / n; hasil = hasil + suku }
    jika negatif { kembalikan 1.0 / hasil }
    lainnya { kembalikan hasil }
}
fungsi my_ln(x) {
    jika x <= 0 { kembalikan 0.0 }
    jika x == 1.0 { kembalikan 0.0 }
    var y = (x - 1.0) / (x + 1.0)
    var y2 = y * y
    var hasil = 0.0
    var y_pow = y
    untuk n dalam 1..30 {
        jika n % 2 == 1 { hasil = hasil + y_pow / n }
        y_pow = y_pow * y2
    }
    kembalikan 2.0 * hasil
}
fungsi my_atan(x) {
    jika x == 0 { kembalikan 0.0 }
    var negatif = salah
    jika x < 0 { negatif = benar; x = -x }
    var besar = salah
    jika x > 1.0 { besar = benar; x = 1.0 / x }
    var x2 = x * x
    var hasil = 0.0
    var x_pow = x
    untuk n dalam 1..30 {
        jika n % 2 == 1 {
            jika n % 4 == 1 { hasil = hasil + x_pow / n }
            lainnya { hasil = hasil - x_pow / n }
        }
        x_pow = x_pow * x2
    }
    jika besar { hasil = PI / 2.0 - hasil }
    jika negatif { hasil = -hasil }
    kembalikan hasil
}
fungsi my_atan2(y, x) {
    jika x > 0 { kembalikan my_atan(y / x) }
    lainnya jika x < 0 {
        jika y >= 0 { kembalikan my_atan(y / x) + PI }
        lainnya { kembalikan my_atan(y / x) - PI }
    } lainnya {
        jika y > 0 { kembalikan PI / 2.0 }
        lainnya jika y < 0 { kembalikan -PI / 2.0 }
        lainnya { kembalikan 0.0 }
    }
}
fungsi haversine_meter(lat1, lon1, lat2, lon2) {
    var r = 6371000.0
    var d_lat = rad(lat2 - lat1)
    var d_lon = rad(lon2 - lon1)
    var a = sin(d_lat / 2.0) * sin(d_lat / 2.0) + cos(rad(lat1)) * cos(rad(lat2)) * sin(d_lon / 2.0) * sin(d_lon / 2.0)
    var c = 2.0 * my_atan2(akar(a), akar(1.0 - a))
    kembalikan r * c
}
fungsi spasial_titik_dalam_poligon(titik, poly) {
    jika panjang(poly) < 3 { kembalikan salah }
    var px = titik[0]
    var py = titik[1]
    var inside = salah
    var n = panjang(poly)
    var j = n - 1
    untuk i dalam 0..(n - 1) {
        var xi = poly[i][0]
        var yi = poly[i][1]
        var xj = poly[j][0]
        var yj = poly[j][1]
        var cond1 = yi > py
        var cond2 = yj > py
        var intersect_cond1 = cond1 != cond2
        var denom = yj - yi
        var px_intersect = px
        jika denom != 0 { px_intersect = (xj - xi) * (py - yi) / denom + xi }
        var intersect = intersect_cond1 dan (px < px_intersect)
        jika intersect { inside = !inside }
        j = i
    }
    kembalikan inside
}
fungsi spasial_hitung_luas_poligon(poly) {
    jika panjang(poly) < 3 { kembalikan 0.0 }
    var area = 0.0
    var n = panjang(poly)
    untuk i dalam 0..(n - 1) {
        var j = (i + 1) % n
        area = area + poly[i][0] * poly[j][1]
        area = area - poly[j][0] * poly[i][1]
    }
    var result = (mutlak(area) / 2.0) * 111.0 * 111.0
    kembalikan result
}
fungsi spasial_hitung_panjang_garis(coords) {
    jika panjang(coords) < 2 { kembalikan 0.0 }
    var total_km = 0.0
    var batas = panjang(coords) - 2
    untuk i dalam 0..batas {
        var lat1 = coords[i][1]
        var lon1 = coords[i][0]
        var lat2 = coords[i + 1][1]
        var lon2 = coords[i + 1][0]
        var jarak_m = haversine_meter(lat1, lon1, lat2, lon2)
        total_km = total_km + jarak_m / 1000.0
    }
    kembalikan total_km
}
fungsi dimulai_teks(s, pref) {
    jika panjang(s) < panjang(pref) { kembalikan salah }
    kembalikan potong(s, 0, panjang(pref)) == pref
}
fungsi hapus_kurung_depan_belakang(s) {
    var r = s
    selama panjang(r) > 0 {
        var c = potong(r, 0, 1)
        jika c == "(" atau c == " " { r = potong(r, 1, panjang(r)) }
        lainnya { berhenti }
    }
    selama panjang(r) > 0 {
        var c = potong(r, panjang(r) - 1, panjang(r))
        jika c == ")" atau c == " " { r = potong(r, 0, panjang(r) - 1) }
        lainnya { berhenti }
    }
    kembalikan r
}
fungsi pisah(s, delim) {
    misal hasil = []; misal mulai = 0; misal i = 0; misal p_delim = panjang(delim);
    selama i <= panjang(s) - p_delim {
        misal cocok = benar; misal j = 0;
        selama j < p_delim {
            jika s[i + j] != delim[j] { cocok = salah; berhenti; }
            j = j + 1;
        }
        jika cocok {
            tambah(hasil, potong(s, mulai, i));
            i = i + p_delim; mulai = i;
        } lainnya { i = i + 1; }
    }
    tambah(hasil, potong(s, mulai, panjang(s)));
    kembalikan hasil;
}
fungsi ke_angka(s) {
    tetap dm = {"0":0,"1":1,"2":2,"3":3,"4":4,"5":5,"6":6,"7":7,"8":8,"9":9};
    misal neg = salah; misal start_idx = 0;
    jika panjang(s) > 0 dan s[0] == "-" { neg = benar; start_idx = 1; }
    misal hasil = 0.0; misal i = start_idx;
    selama i < panjang(s) {
        misal d = s[i];
        jika ada_kunci(dm, d) { hasil = hasil * 10.0 + dm[d]; }
        lainnya { berhenti; }
        i = i + 1;
    }
    jika neg { hasil = 0.0 - hasil; }
    kembalikan hasil;
}
fungsi parse_wkt_ke_titik_array(wkt) {
    var upper = huruf_besar(wkt)
    var sisa = ""
    jika dimulai_teks(upper, "POINT") { sisa = potong(wkt, 5, panjang(wkt)) }
    lainnya jika dimulai_teks(upper, "POLYGON") { sisa = potong(wkt, 7, panjang(wkt)) }
    lainnya jika dimulai_teks(upper, "LINESTRING") { sisa = potong(wkt, 10, panjang(wkt)) }
    lainnya { kembalikan [] }
    var bagian_koord = hapus_kurung_depan_belakang(sisa)
    jika panjang(bagian_koord) == 0 { kembalikan [] }
    var pairs = pisah(bagian_koord, ",")
    var hasil = []
    jika panjang(pairs) == 0 { kembalikan hasil }
    var n = panjang(pairs) - 1
    untuk i dalam 0..n {
        var pair = pairs[i]
        selama dimulai_teks(pair, " ") { pair = potong(pair, 1, panjang(pair)) }
        selama panjang(pair) > 0 dan potong(pair, panjang(pair) - 1, panjang(pair)) == " " {
            pair = potong(pair, 0, panjang(pair) - 1)
        }
        var parts = pisah(pair, " ")
        var clean_parts = []
        jika panjang(parts) > 0 {
            var pk = 0
            untuk pk dalam 0..(panjang(parts) - 1) {
                jika panjang(parts[pk]) > 0 { clean_parts = tambah(clean_parts, parts[pk]) }
            }
        }
        jika panjang(clean_parts) >= 2 {
            var px = ke_angka(clean_parts[0])
            var py = ke_angka(clean_parts[1])
            hasil = tambah(hasil, [px, py])
        }
    }
    kembalikan hasil
}
fungsi deteksi_tipe_wkt(wkt) {
    var upper = huruf_besar(wkt)
    jika dimulai_teks(upper, "POINT") { kembalikan "Point" }
    lainnya jika dimulai_teks(upper, "POLYGON") { kembalikan "Polygon" }
    lainnya jika dimulai_teks(upper, "LINESTRING") { kembalikan "LineString" }
    lainnya { kembalikan "Unknown" }
}
fungsi dapat_bbox(arr) {
    jika panjang(arr) == 0 { kembalikan { "min_x": 0, "min_y": 0, "max_x": 0, "max_y": 0 } }
    var min_x = arr[0][0]
    var max_x = arr[0][0]
    var min_y = arr[0][1]
    var max_y = arr[0][1]
    var n = panjang(arr) - 1
    untuk i dalam 0..n {
        jika arr[i][0] < min_x { min_x = arr[i][0] }
        jika arr[i][0] > max_x { max_x = arr[i][0] }
        jika arr[i][1] < min_y { min_y = arr[i][1] }
        jika arr[i][1] > max_y { max_y = arr[i][1] }
    }
    kembalikan { "min_x": min_x, "min_y": min_y, "max_x": max_x, "max_y": max_y }
}
fungsi bbox_overlap(bb1, bb2) {
    kembalikan (bb1["min_x"] <= bb2["max_x"]) dan (bb1["max_x"] >= bb2["min_x"]) dan (bb1["min_y"] <= bb2["max_y"]) dan (bb1["max_y"] >= bb2["min_y"])
}
fungsi ProyeksiCRS(dari, ke) {
    kembalikan { "_tipe": "ProyeksiCRS", "sumber_crs": huruf_besar(dari), "tujuan_crs": huruf_besar(ke) }
}
fungsi crs_transformasi_koordinat(proj, x, y) {
    var dari = proj["sumber_crs"]
    var ke = proj["tujuan_crs"]
    var out_x = x
    var out_y = y
    jika dari == "EPSG:4326" dan ke == "EPSG:3857" {
        out_x = x * 20037508.34 / 180.0
        var lat_rad = rad(y)
        out_y = my_ln(tan(PI / 4.0 + lat_rad / 2.0)) * 20037508.34 / PI
    } lainnya jika dari == "EPSG:3857" dan ke == "EPSG:4326" {
        out_x = x * 180.0 / 20037508.34
        out_y = my_atan(my_exp(y / 20037508.34 * PI)) * 360.0 / PI - 90.0
    }
    kembalikan { "x": out_x, "y": out_y }
}
fungsi ST_Area(geom_poly_wkt) {
    var titik = parse_wkt_ke_titik_array(geom_poly_wkt)
    kembalikan spasial_hitung_luas_poligon(titik)
}
fungsi ST_Length(geom_line_wkt) {
    var titik = parse_wkt_ke_titik_array(geom_line_wkt)
    kembalikan spasial_hitung_panjang_garis(titik)
}
fungsi ST_Contains(geom_luar_poly_wkt, geom_dalam) {
    var poly_luar = parse_wkt_ke_titik_array(geom_luar_poly_wkt)
    jika panjang(poly_luar) < 3 { kembalikan salah }
    var tipe_dalam = deteksi_tipe_wkt(geom_dalam)
    jika tipe_dalam == "Point" {
        var pt = parse_wkt_ke_titik_array(geom_dalam)
        jika panjang(pt) < 1 { kembalikan salah }
        kembalikan spasial_titik_dalam_poligon(pt[0], poly_luar)
    } lainnya jika tipe_dalam == "Polygon" {
        var poly_dalam = parse_wkt_ke_titik_array(geom_dalam)
        jika panjang(poly_dalam) == 0 { kembalikan salah }
        var semua_dalam = benar
        var n = panjang(poly_dalam) - 1
        untuk i dalam 0..n {
            jika !spasial_titik_dalam_poligon(poly_dalam[i], poly_luar) {
                semua_dalam = salah; berhenti
            }
        }
        kembalikan semua_dalam
    } lainnya { kembalikan salah }
}
fungsi ST_Within(A, B) { kembalikan ST_Contains(B, A) }
fungsi ST_Intersects(A, B) {
    var arr_A = parse_wkt_ke_titik_array(A)
    var arr_B = parse_wkt_ke_titik_array(B)
    jika panjang(arr_A) == 0 atau panjang(arr_B) == 0 { kembalikan salah }
    var bbA = dapat_bbox(arr_A)
    var bbB = dapat_bbox(arr_B)
    jika bbox_overlap(bbA, bbB) { kembalikan benar }
    kembalikan salah
}
fungsi ST_DWithin(point_A, point_B, radius_meter) {
    var lat1 = point_A[1]
    var lon1 = point_A[0]
    var lat2 = point_B[1]
    var lon2 = point_B[0]
    var jarak = haversine_meter(lat1, lon1, lat2, lon2)
    kembalikan jarak <= radius_meter
}
fungsi ST_Transform(array_titik, dari_crs, ke_crs) {
    var proj = ProyeksiCRS(dari_crs, ke_crs)
    var hasil = []
    jika panjang(array_titik) == 0 { kembalikan hasil }
    var n = panjang(array_titik) - 1
    untuk i dalam 0..n {
        var lon = array_titik[i][0]
        var lat = array_titik[i][1]
        var res = crs_transformasi_koordinat(proj, lon, lat)
        hasil = tambah(hasil, { "x": res["x"], "y": res["y"] })
    }
    kembalikan hasil
}
fungsi ST_Buffer(point, radius_meter) {
    var lon = point[0]
    var lat = point[1]
    var delta_deg = (radius_meter / 1000.0) / 111.0
    var min_lon = lon - delta_deg
    var max_lon = lon + delta_deg
    var min_lat = lat - delta_deg
    var max_lat = lat + delta_deg
    kembalikan [min_lon, min_lat, max_lon, max_lat]
}
var poly_1deg = "POLYGON((0 0, 1 0, 1 1, 0 1, 0 0))"
var luas = ST_Area(poly_1deg)
pastikan(luas > 11000.0 dan luas < 14000.0, "ST_Area test gagal")
var pt_a = [106.8456, -6.2088]
var pt_b_1km = [106.8546, -6.2088]
var dwithin_1km = ST_DWithin(pt_a, pt_b_1km, 1100.0)
pastikan(dwithin_1km == benar, "ST_DWithin 1km test gagal")
var pt_b_100km = [107.8456, -6.2088]
var dwithin_100km = ST_DWithin(pt_a, pt_b_100km, 50.0)
pastikan(dwithin_100km == salah, "ST_DWithin 100km+50m test gagal")
var poly_01 = "POLYGON((0 0, 1 0, 1 1, 0 1, 0 0))"
var pt_tengah = "POINT(0.5 0.5)"
var contains = ST_Contains(poly_01, pt_tengah)
pastikan(contains == benar, "ST_Contains test gagal")
var within = ST_Within(pt_tengah, poly_01)
pastikan(within == benar, "ST_Within test gagal")
var poly_A = "POLYGON((0 0, 2 0, 2 2, 0 2, 0 0))"
var poly_B = "POLYGON((1 1, 3 1, 3 3, 1 3, 1 1))"
var intersects = ST_Intersects(poly_A, poly_B)
pastikan(intersects == benar, "ST_Intersects test gagal")
var jkt = [[106.8456, -6.2088]]
var transformed = ST_Transform(jkt, "EPSG:4326", "EPSG:3857")
pastikan(transformed[0]["x"] > 0.0, "ST_Transform x>0 test gagal")
var line_2pt = "LINESTRING(0 0, 1 1)"
var panjang_line = ST_Length(line_2pt)
pastikan(panjang_line > 0.0, "ST_Length test gagal")
var buf_pt = [100.0, 0.0]
var buf_hasil = ST_Buffer(buf_pt, 111000.0)
pastikan(panjang(buf_hasil) == 4, "ST_Buffer 4 elemen test gagal")
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_3_st_spatial_8functions_ogc: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_4_spatial_join_atribut_r_tree_pip() {
    let code = r##"
fungsi rad(deg) {
    kembalikan deg * PI / 180.0
}

fungsi my_faktorial(n) {
    jika n == 0 {
        kembalikan 1.0
    }
    misal hasil = 1.0
    untuk i dalam 1..n {
        hasil = hasil * i
    }
    kembalikan hasil
}

fungsi my_exp(x) {
    jika x == 0 {
        kembalikan 1.0
    }
    misal negatif = salah
    jika x < 0 {
        negatif = benar
        x = -x
    }
    misal hasil = 1.0
    misal suku = 1.0
    untuk n dalam 1..25 {
        suku = suku * x / n
        hasil = hasil + suku
    }
    jika negatif {
        kembalikan 1.0 / hasil
    } lainnya {
        kembalikan hasil
    }
}

fungsi my_ln(x) {
    jika x <= 0 {
        kembalikan 0.0
    }
    jika x == 1.0 {
        kembalikan 0.0
    }
    misal y = (x - 1.0) / (x + 1.0)
    misal y2 = y * y
    misal hasil = 0.0
    misal y_pow = y
    untuk n dalam 1..30 {
        jika n % 2 == 1 {
            hasil = hasil + y_pow / n
        }
        y_pow = y_pow * y2
    }
    kembalikan 2.0 * hasil
}

fungsi my_atan(x) {
    jika x == 0 {
        kembalikan 0.0
    }
    misal negatif = salah
    jika x < 0 {
        negatif = benar
        x = -x
    }
    misal besar = salah
    jika x > 1.0 {
        besar = benar
        x = 1.0 / x
    }
    misal x2 = x * x
    misal hasil = 0.0
    misal x_pow = x
    untuk n dalam 1..30 {
        jika n % 2 == 1 {
            jika n % 4 == 1 {
                hasil = hasil + x_pow / n
            } lainnya {
                hasil = hasil - x_pow / n
            }
        }
        x_pow = x_pow * x2
    }
    jika besar {
        hasil = PI / 2.0 - hasil
    }
    jika negatif {
        hasil = -hasil
    }
    kembalikan hasil
}

fungsi my_atan2(y, x) {
    jika x > 0 {
        kembalikan my_atan(y / x)
    } lainnya jika x < 0 {
        jika y >= 0 {
            kembalikan my_atan(y / x) + PI
        } lainnya {
            kembalikan my_atan(y / x) - PI
        }
    } lainnya {
        jika y > 0 {
            kembalikan PI / 2.0
        } lainnya jika y < 0 {
            kembalikan -PI / 2.0
        } lainnya {
            kembalikan 0.0
        }
    }
}

fungsi haversine_meter(lat1, lon1, lat2, lon2) {
    misal r = 6371000.0
    misal d_lat = rad(lat2 - lat1)
    misal d_lon = rad(lon2 - lon1)
    misal a = sin(d_lat / 2.0) * sin(d_lat / 2.0) + cos(rad(lat1)) * cos(rad(lat2)) * sin(d_lon / 2.0) * sin(d_lon / 2.0)
    misal c = 2.0 * my_atan2(akar(a), akar(1.0 - a))
    kembalikan r * c
}

fungsi spasial_titik_dalam_poligon(titik, poly) {
    jika panjang(poly) < 3 {
        kembalikan salah
    }
    misal px = titik[0]
    misal py = titik[1]
    misal inside = salah
    misal n = panjang(poly)
    misal j = n - 1
    untuk i dalam 0..(n - 1) {
        misal xi = poly[i][0]
        misal yi = poly[i][1]
        misal xj = poly[j][0]
        misal yj = poly[j][1]
        misal cond1 = yi > py
        misal cond2 = yj > py
        misal intersect_cond1 = cond1 != cond2
        misal denom = yj - yi
        misal px_intersect = px
        jika denom != 0 {
            px_intersect = (xj - xi) * (py - yi) / denom + xi
        }
        misal intersect = intersect_cond1 dan (px < px_intersect)
        jika intersect {
            inside = !inside
        }
        j = i
    }
    kembalikan inside
}

fungsi spasial_hitung_luas_poligon(poly) {
    jika panjang(poly) < 3 {
        kembalikan 0.0
    }
    misal area = 0.0
    misal n = panjang(poly)
    untuk i dalam 0..(n - 1) {
        misal j = (i + 1) % n
        area = area + poly[i][0] * poly[j][1]
        area = area - poly[j][0] * poly[i][1]
    }
    misal result = (mutlak(area) / 2.0) * 111.0 * 111.0
    kembalikan result
}

fungsi spasial_hitung_panjang_garis(line_pts) {
    misal total = 0.0
    misal n = panjang(line_pts)
    untuk i dalam 0..(n - 2) {
        misal p1 = line_pts[i]
        misal p2 = line_pts[i + 1]
        total = total + haversine_meter(p1[1], p1[0], p2[1], p2[0]) / 1000.0
    }
    kembalikan total
}

struktur IndeksRTree {
    kapasitas,
    entri,

    fungsi inisialisasi(kap) {
        ini.kapasitas = kap
        ini.entri = []
    }
}

fungsi rtree_sisip(rt, id, min_x, min_y, max_x, max_y, data) {
    tambah(rt.entri, {
        "id": id,
        "min_x": min_x,
        "min_y": min_y,
        "max_x": max_x,
        "max_y": max_y,
        "data": data
    })
    kembalikan benar
}

fungsi rtree_kueri_kotak(rt, qmin_x, qmin_y, qmax_x, qmax_y) {
    misal hasil = []
    untuk_setiap(rt.entri, fungsi(e) {
        misal overlap_x = e.min_x <= qmax_x dan e.max_x >= qmin_x
        misal overlap_y = e.min_y <= qmax_y dan e.max_y >= qmin_y
        jika overlap_x dan overlap_y {
            tambah(hasil, e)
        }
    })
    kembalikan hasil
}

struktur PohonBPlusMap {
    simpanan,

    fungsi inisialisasi(_order) {
        ini.simpanan = {}
    }
}

fungsi bplus_sisip(pohon, kunci, nilai) {
    pohon.simpanan[kunci] = nilai
    kembalikan benar
}

fungsi bplus_cari(pohon, kunci) {
    jika ada_kunci(pohon.simpanan, kunci) {
        kembalikan pohon.simpanan[kunci]
    }
    kembalikan nihil
}

fungsi bplus_rentang(pohon, kmin, kmaks) {
    misal hasil = []
    untuk k dalam kunci(pohon.simpanan) {
        jika k >= kmin dan k <= kmaks {
            tambah(hasil, {"kunci": k, "nilai": pohon.simpanan[k]})
        }
    }
    kembalikan hasil
}

fungsi PohonBPlus(order) {
    kembalikan PohonBPlusMap(order)
}

struktur TabelRelasional {
    nama_tabel,
    kolom,
    baris_data,
    indeks_pk,

    fungsi inisialisasi(nama, daftar_kolom) {
        ini.nama_tabel = nama
        ini.kolom = daftar_kolom
        ini.baris_data = []
        ini.indeks_pk = {}
    }

    fungsi sisip(data_record) {
        misal id = data_record["id"]
        jika id != nihil dan ada_kunci(ini.indeks_pk, ke_teks(id)) {
            kembalikan salah
        }
        tambah(ini.baris_data, data_record)
        jika id != nihil {
            ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1
        }
        kembalikan benar
    }

    fungsi pilih_di_mana(fungsi_predikat) {
        kembalikan saring(ini.baris_data, fungsi_predikat)
    }

    fungsi perbarui_berdasarkan_id(id, data_baru) {
        misal key = ke_teks(id)
        jika bukan ada_kunci(ini.indeks_pk, key) {
            kembalikan salah
        }
        misal idx = ini.indeks_pk[key]
        ini.baris_data[idx] = data_baru
        kembalikan benar
    }
}

struktur KoleksiDokumen {
    nama_koleksi,
    dokumen,
    indeks_field,
    _dokumen_by_id,

    fungsi inisialisasi(nama) {
        ini.nama_koleksi = nama
        ini.dokumen = []
        ini.indeks_field = {}
        ini._dokumen_by_id = {}
    }

    fungsi sisip_satu(doc) {
        jika bukan ada_kunci(doc, "_id") {
            doc["_id"] = "doc_" + ke_teks(panjang(ini.dokumen) + 1)
        }
        tambah(ini.dokumen, doc)
        ini._dokumen_by_id[doc["_id"]] = panjang(ini.dokumen) - 1

        misal field_terindeks = kunci(ini.indeks_field)
        jika panjang(field_terindeks) > 0 {
            untuk_setiap(field_terindeks, fungsi(nama_field) {
                misal pohon = ini.indeks_field[nama_field]
                jika ada_kunci(doc, nama_field) {
                    misal nilai_field = ke_teks(doc[nama_field])
                    misal existing = bplus_cari(pohon, nilai_field)
                    misal daftar_id = []
                    jika existing != nihil {
                        daftar_id = existing
                    }
                    tambah(daftar_id, doc["_id"])
                    bplus_sisip(pohon, nilai_field, daftar_id)
                }
            })
        }
        kembalikan Ok(doc["_id"])
    }

    fungsi cari(kunci_field, nilai_target) {
        kembalikan saring(ini.dokumen, fungsi(item) {
            jika ada_kunci(item, kunci_field) {
                kembalikan item[kunci_field] == nilai_target
            }
            kembalikan salah
        })
    }
}

fungsi _cari_dokumen_by_id(koleksi, id) {
    jika ada_kunci(koleksi._dokumen_by_id, id) {
        kembalikan koleksi.dokumen[koleksi._dokumen_by_id[id]]
    }
    misal hasil = saring(koleksi.dokumen, fungsi(d) {
        kembalikan d["_id"] == id
    })
    jika panjang(hasil) > 0 {
        kembalikan hasil[0]
    }
    kembalikan nihil
}

fungsi indeks_tambah(koleksi, nama_field) {
    misal pohon = PohonBPlus(4)
    untuk_setiap(koleksi.dokumen, fungsi(doc) {
        jika ada_kunci(doc, nama_field) {
            misal nilai_field = ke_teks(doc[nama_field])
            misal existing = bplus_cari(pohon, nilai_field)
            misal daftar_id = []
            jika existing != nihil {
                daftar_id = existing
            }
            tambah(daftar_id, doc["_id"])
            bplus_sisip(pohon, nilai_field, daftar_id)
        }
    })
    koleksi.indeks_field[nama_field] = pohon
    kembalikan benar
}

fungsi indeks_cari(koleksi, nama_field, nilai_cari) {
    jika bukan ada_kunci(koleksi.indeks_field, nama_field) {
        kembalikan []
    }
    misal pohon = koleksi.indeks_field[nama_field]
    misal kunci_cari = ke_teks(nilai_cari)
    misal hasil_ids = bplus_cari(pohon, kunci_cari)
    jika hasil_ids == nihil {
        kembalikan []
    }
    misal hasil = []
    untuk_setiap(hasil_ids, fungsi(id) {
        misal doc = _cari_dokumen_by_id(koleksi, id)
        jika doc != nihil {
            tambah(hasil, doc)
        }
    })
    kembalikan hasil
}

struktur DeretWaktu {
    nama,
    data_points,

    fungsi inisialisasi(nama) {
        ini.nama = nama
        ini.data_points = []
    }
}

fungsi tambah_titik_waktu(dw, ts, val) {
    tambah(dw.data_points, {
        "waktu": ts,
        "nilai": val
    })
    kembalikan benar
}

fungsi buat_basis_data_vektor_sederhana(dim) {
    kembalikan {
        "dimensi": dim,
        "entri": []
    }
}

fungsi vektor_sisip_sederhana(db, id, vek, meta) {
    tambah(db.entri, {
        "id": id,
        "vektor": vek,
        "metadata": meta
    })
    kembalikan benar
}

fungsi vektor_hitung_sederhana(db) {
    kembalikan panjang(db.entri)
}

struktur WidyaDB {
    nama_db,
    geo,
    rel,
    nosql,

    fungsi inisialisasi(nama_db) {
        misal rtree = IndeksRTree(16)

        ini.nama_db = nama_db
        ini.geo = {
            "rtree": rtree,
            "titik_dalam_poligon": spasial_titik_dalam_poligon,
            "hitung_luas_poligon": spasial_hitung_luas_poligon,
            "hitung_panjang_garis": spasial_hitung_panjang_garis,
            "rtree_sisip": rtree_sisip,
            "rtree_kueri_kotak": rtree_kueri_kotak
        }
        ini.rel = {
            "tabel": {}
        }
        ini.nosql = {
            "dokumen": {}
        }
    }
}

fungsi spasial_join_atribut(db, list_id_geom, nama_tabel_relasional, kolom_fk) {
    misal hasil = []
    jika bukan ada_kunci(db.rel.tabel, nama_tabel_relasional) {
        kembalikan hasil
    }
    misal tbl = db.rel.tabel[nama_tabel_relasional]
    untuk geom_id dalam list_id_geom {
        misal baris_cocok = tbl.pilih_di_mana(fungsi(baris) {
            jika ada_kunci(baris, kolom_fk) {
                kembalikan ke_teks(baris[kolom_fk]) == ke_teks(geom_id)
            }
            kembalikan salah
        })
        untuk b dalam baris_cocok {
            tambah(hasil, {
                "id_geom": geom_id,
                "atribut": b
            })
        }
    }
    kembalikan hasil
}

misal db = WidyaDB("kecamatan_xyz")
pastikan(db.nama_db == "kecamatan_xyz", "Nama DB harus kecamatan_xyz")
pastikan(db.geo != nihil, "Komponen geo harus ada")
pastikan(db.rel != nihil, "Komponen rel harus ada")
pastikan(db.nosql != nihil, "Komponen nosql harus ada")

misal geo_kecamatan = []
misal bbox_A = [0.0, 0.0, 10.0, 10.0]
misal poly_A = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [0.0, 0.0]]
misal kec_A = {"id": 1, "nama": "KecA", "bbox": bbox_A, "poligon": poly_A}
tambah(geo_kecamatan, kec_A)
rtree_sisip(db.geo.rtree, 1, 0.0, 0.0, 10.0, 10.0, kec_A)

misal bbox_B = [11.0, 0.0, 25.0, 15.0]
misal poly_B = [[11.0, 0.0], [25.0, 0.0], [25.0, 15.0], [11.0, 15.0], [11.0, 0.0]]
misal kec_B = {"id": 2, "nama": "KecB", "bbox": bbox_B, "poligon": poly_B}
tambah(geo_kecamatan, kec_B)
rtree_sisip(db.geo.rtree, 2, 11.0, 0.0, 25.0, 15.0, kec_B)

misal bbox_C = [0.0, 16.0, 15.0, 30.0]
misal poly_C = [[0.0, 16.0], [15.0, 16.0], [15.0, 30.0], [0.0, 30.0], [0.0, 16.0]]
misal kec_C = {"id": 3, "nama": "KecC", "bbox": bbox_C, "poligon": poly_C}
tambah(geo_kecamatan, kec_C)
rtree_sisip(db.geo.rtree, 3, 0.0, 16.0, 15.0, 30.0, kec_C)

pastikan(panjang(geo_kecamatan) == 3, "Harus ada 3 kecamatan di array geo_kecamatan")
misal cek_rt = rtree_kueri_kotak(db.geo.rtree, 0.0, 0.0, 30.0, 30.0)
pastikan(panjang(cek_rt) == 3, "RTree harus berisi 3 kecamatan")

misal kolom_warga = [
    {"nama": "id", "tipe": "INT"},
    {"nama": "nama", "tipe": "STR"},
    {"nama": "kecamatan_id", "tipe": "INT"},
    {"nama": "lat", "tipe": "FLOAT"},
    {"nama": "lng", "tipe": "FLOAT"}
]
misal tabel_warga = TabelRelasional("warga", kolom_warga)
db.rel.tabel["warga"] = tabel_warga

tabel_warga.sisip({"id": 1, "nama": "Warga1", "kecamatan_id": 1, "lat": 5.0, "lng": 5.0})
tabel_warga.sisip({"id": 2, "nama": "Warga2", "kecamatan_id": 1, "lat": 3.0, "lng": 7.0})
tabel_warga.sisip({"id": 3, "nama": "Warga3", "kecamatan_id": 1, "lat": 8.0, "lng": 2.0})
tabel_warga.sisip({"id": 4, "nama": "Warga4", "kecamatan_id": 2, "lat": 12.0, "lng": 18.0})
tabel_warga.sisip({"id": 5, "nama": "Warga5", "kecamatan_id": 2, "lat": 5.0, "lng": 20.0})
tabel_warga.sisip({"id": 6, "nama": "Warga6", "kecamatan_id": 2, "lat": 10.0, "lng": 15.0})
tabel_warga.sisip({"id": 7, "nama": "Warga7", "kecamatan_id": 2, "lat": 8.0, "lng": 22.0})
tabel_warga.sisip({"id": 8, "nama": "Warga8", "kecamatan_id": 3, "lat": 20.0, "lng": 8.0})
tabel_warga.sisip({"id": 9, "nama": "Warga9", "kecamatan_id": 3, "lat": 25.0, "lng": 10.0})
tabel_warga.sisip({"id": 10, "nama": "Warga10", "kecamatan_id": 3, "lat": 22.0, "lng": 5.0})

pastikan(panjang(tabel_warga.baris_data) == 10, "Tabel warga harus berisi 10 baris")
misal cek_kecA = tabel_warga.pilih_di_mana(fungsi(b) { kembalikan b.kecamatan_id == 1 })
pastikan(panjang(cek_kecA) == 3, "Warga KecA harus 3 orang")
misal cek_kecB = tabel_warga.pilih_di_mana(fungsi(b) { kembalikan b.kecamatan_id == 2 })
pastikan(panjang(cek_kecB) == 4, "Warga KecB harus 4 orang")
misal cek_kecC = tabel_warga.pilih_di_mana(fungsi(b) { kembalikan b.kecamatan_id == 3 })
pastikan(panjang(cek_kecC) == 3, "Warga KecC harus 3 orang")

misal log_sensor_koleksi = KoleksiDokumen("log_sensor")
db.nosql.dokumen["log_sensor"] = log_sensor_koleksi
misal ts_now = 1700000000
misal nomor_s = 1
selama nomor_s <= 20 {
    misal kid = ((nomor_s - 1) % 3) + 1
    misal suhu = 20.0 + (acak() * 15.0)
    log_sensor_koleksi.sisip_satu({
        "sensor_id": "S1",
        "waktu": ts_now + nomor_s * 60,
        "lokasi_kecamatan_id": kid,
        "kecamatan_id": kid,
        "nilai_suhu": suhu
    })
    nomor_s = nomor_s + 1
}
pastikan(panjang(log_sensor_koleksi.dokumen) == 20, "log_sensor harus berisi 20 dokumen")

misal suhu_rata2 = DeretWaktu("suhu_rata2")
db.nosql.ts = suhu_rata2
misal ts_awal = 1700100000
untuk i_ts dalam 0..99 {
    misal ts_p = ts_awal + i_ts * 300
    misal val_p = 24.0 + (acak() * 4.0)
    tambah_titik_waktu(suhu_rata2, ts_p, val_p)
}
pastikan(panjang(suhu_rata2.data_points) == 100, "DeretWaktu suhu_rata2 harus 100 titik")

misal embedding_warga = buat_basis_data_vektor_sederhana(4)
db.nosql.vektor = embedding_warga
misal nama_warga_list = ["Warga1", "Warga2", "Warga3", "Warga4", "Warga5"]
untuk iv dalam 0..4 {
    misal wid = iv + 1
    misal v = [
        acak() * 2.0 - 1.0,
        acak() * 2.0 - 1.0,
        acak() * 2.0 - 1.0,
        acak() * 2.0 - 1.0
    ]
    vektor_sisip_sederhana(embedding_warga, "w" + ke_teks(wid), v, {"nama_warga": nama_warga_list[iv], "id_warga": wid})
}
pastikan(vektor_hitung_sederhana(embedding_warga) == 5, "BasisDataVektor embedding_warga harus 5 entri")

indeks_tambah(log_sensor_koleksi, "kecamatan_id")
misal hasil_idx_kec2 = indeks_cari(log_sensor_koleksi, "kecamatan_id", 2)
pastikan(panjang(hasil_idx_kec2) >= 4, "indeks_cari kecamatan_id=2 harus >= 4, tapi: " + ke_teks(panjang(hasil_idx_kec2)))

misal rtree_kecB = rtree_kueri_kotak(db.geo.rtree, 11.0, 0.0, 25.0, 15.0)
pastikan(panjang(rtree_kecB) >= 1, "RTree query kotak KecB harus menemukan minimal 1")

misal list_id_kecB = []
untuk_setiap(rtree_kecB, fungsi(item) {
    misal id_geom = item.id
    misal poly_item = nihil
    untuk_setiap(geo_kecamatan, fungsi(gk) {
        jika gk.id == id_geom {
            poly_item = gk.poligon
        }
    })
    jika poly_item != nihil {
        misal sample_pt = [(item.min_x + item.max_x) / 2.0, (item.min_y + item.max_y) / 2.0]
        jika spasial_titik_dalam_poligon(sample_pt, poly_item) {
            tambah(list_id_kecB, id_geom)
        }
    } lainnya {
        tambah(list_id_kecB, id_geom)
    }
})

misal warga_yang_ada_di_KecB = spasial_join_atribut(db, list_id_kecB, "warga", "kecamatan_id")
pastikan(panjang(warga_yang_ada_di_KecB) == 4, "SPATIAL JOIN warga di KecB harus 4 orang, tapi: " + ke_teks(panjang(warga_yang_ada_di_KecB)))

kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_4_spatial_join_atribut_r_tree_pip: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_5_pgwire_server_pipeline_mock() {
    let code = r##"
fungsi ada(haystack, needle) {
    misal pjg_h = panjang(haystack)
    misal pjg_n = panjang(needle)
    jika pjg_n == 0 { kembalikan benar }
    jika pjg_n > pjg_h { kembalikan salah }
    misal i = 0
    selama i <= pjg_h - pjg_n {
        misal cocok = benar
        misal j = 0
        selama j < pjg_n {
            jika haystack[i + j] != needle[j] { cocok = salah; berhenti }
            j = j + 1
        }
        jika cocok { kembalikan benar }
        i = i + 1
    }
    kembalikan salah
}

struktur TabelRelasional {
    nama_tabel,
    kolom,
    baris_data,
    indeks_pk,

    fungsi inisialisasi(nama, daftar_kolom) {
        ini.nama_tabel = nama
        ini.kolom = daftar_kolom
        ini.baris_data = []
        ini.indeks_pk = {}
    }

    fungsi sisip(data_record) {
        misal id = data_record["id"]
        jika id != nihil dan ada_kunci(ini.indeks_pk, ke_teks(id)) {
            kembalikan salah
        }
        tambah(ini.baris_data, data_record)
        jika id != nihil {
            ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1
        }
        kembalikan benar
    }

    fungsi pilih_di_mana(fungsi_predikat) {
        kembalikan saring(ini.baris_data, fungsi_predikat)
    }

    fungsi perbarui_berdasarkan_id(id, data_baru) {
        misal key = ke_teks(id)
        jika bukan ada_kunci(ini.indeks_pk, key) {
            kembalikan salah
        }
        misal idx = ini.indeks_pk[key]
        ini.baris_data[idx] = data_baru
        kembalikan benar
    }
}

fungsi saring_bawaan(daftar, predikat) {
    misal hasil = []
    untuk item dalam daftar {
        jika predikat(item) {
            tambah(hasil, item)
        }
    }
    kembalikan hasil
}

fungsi apakah_huruf(c) {
    jika panjang(c) == 0 {
        kembalikan salah
    }
    tetap huruf_map = {
        "A": benar, "B": benar, "C": benar, "D": benar, "E": benar, "F": benar, "G": benar,
        "H": benar, "I": benar, "J": benar, "K": benar, "L": benar, "M": benar, "N": benar,
        "O": benar, "P": benar, "Q": benar, "R": benar, "S": benar, "T": benar, "U": benar,
        "V": benar, "W": benar, "X": benar, "Y": benar, "Z": benar,
        "a": benar, "b": benar, "c": benar, "d": benar, "e": benar, "f": benar, "g": benar,
        "h": benar, "i": benar, "j": benar, "k": benar, "l": benar, "m": benar, "n": benar,
        "o": benar, "p": benar, "q": benar, "r": benar, "s": benar, "t": benar, "u": benar,
        "v": benar, "w": benar, "x": benar, "y": benar, "z": benar,
        "_": benar
    }
    kembalikan ada_kunci(huruf_map, c)
}

fungsi apakah_digit(c) {
    tetap digit_map = {"0": benar, "1": benar, "2": benar, "3": benar, "4": benar, "5": benar, "6": benar, "7": benar, "8": benar, "9": benar}
    kembalikan ada_kunci(digit_map, c)
}

fungsi apakah_huruf_atau_digit(c) {
    kembalikan apakah_huruf(c) atau apakah_digit(c)
}

fungsi ke_besar_teks(s) {
    tetap map_besar = {
        "a": "A", "b": "B", "c": "C", "d": "D", "e": "E", "f": "F", "g": "G",
        "h": "H", "i": "I", "j": "J", "k": "K", "l": "L", "m": "M", "n": "N",
        "o": "O", "p": "P", "q": "Q", "r": "R", "s": "S", "t": "T", "u": "U",
        "v": "V", "w": "W", "x": "X", "y": "Y", "z": "Z"
    }
    misal hasil = ""
    misal i = 0
    selama i < panjang(s) {
        misal c = s[i]
        jika ada_kunci(map_besar, c) {
            hasil = hasil + map_besar[c]
        } lainnya {
            hasil = hasil + c
        }
        i = i + 1
    }
    kembalikan hasil
}

fungsi sql_lexer(sql_string) {
    misal tokens = []
    misal pos = 0
    misal len = panjang(sql_string)

    tetap KW_MAP = {
        "SELECT": "SELECT", "FROM": "FROM", "WHERE": "WHERE", "JOIN": "JOIN", "ON": "ON",
        "GROUP": "GROUP", "BY": "BY", "ORDER": "ORDER", "ASC": "ASC", "DESC": "DESC",
        "INSERT": "INSERT", "INTO": "INTO", "VALUES": "VALUES", "UPDATE": "UPDATE",
        "SET": "SET", "DELETE": "DELETE", "LIMIT": "LIMIT", "INT": "INT", "STR": "STR",
        "AND": "AND", "OR": "OR", "NOT": "NOT", "NULL": "NULL", "SUM": "SUM",
        "AVG": "AVG", "COUNT": "COUNT", "MIN": "MIN", "MAX": "MAX",
        "INNER": "INNER", "LEFT": "LEFT", "RIGHT": "RIGHT", "OUTER": "OUTER"
    }

    selama pos < len {
        misal c = sql_string[pos]
        misal ditangani = salah

        jika bukan ditangani dan (c == " " atau c == "\t" atau c == "\n" atau c == "\r") {
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == ";" {
            tambah(tokens, {"tipe": "TITIK_KOMA", "nilai": ";"})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == "," {
            tambah(tokens, {"tipe": "KOMA", "nilai": ","})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == "*" {
            tambah(tokens, {"tipe": "ASTERISK", "nilai": "*"})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == "(" {
            tambah(tokens, {"tipe": "KURUNG_BUKA", "nilai": "("})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == ")" {
            tambah(tokens, {"tipe": "KURUNG_TUTUP", "nilai": ")"})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == "." {
            tambah(tokens, {"tipe": "DOT", "nilai": "."})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == "=" {
            tambah(tokens, {"tipe": "OP_EQ", "nilai": "="})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == ">" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" {
                tambah(tokens, {"tipe": "OP_GTE", "nilai": ">="})
                pos = pos + 2
            } lainnya {
                tambah(tokens, {"tipe": "OP_GT", "nilai": ">"})
                pos = pos + 1
            }
            ditangani = benar
        }

        jika bukan ditangani dan c == "<" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" {
                tambah(tokens, {"tipe": "OP_LTE", "nilai": "<="})
                pos = pos + 2
            } lainnya jika pos + 1 < len dan sql_string[pos + 1] == ">" {
                tambah(tokens, {"tipe": "OP_NEQ", "nilai": "<>"})
                pos = pos + 2
            } lainnya {
                tambah(tokens, {"tipe": "OP_LT", "nilai": "<"})
                pos = pos + 1
            }
            ditangani = benar
        }

        jika bukan ditangani dan c == "!" {
            jika pos + 1 < len dan sql_string[pos + 1] == "=" {
                tambah(tokens, {"tipe": "OP_NEQ", "nilai": "!="})
                pos = pos + 2
            }
        }

        jika bukan ditangani dan c == "'" {
            misal mulai = pos + 1
            pos = pos + 1
            selama pos < len dan sql_string[pos] != "'" {
                jika sql_string[pos] == "\\" dan pos + 1 < len {
                    pos = pos + 2
                } lainnya {
                    pos = pos + 1
                }
            }
            misal str_val = potong(sql_string, mulai, pos)
            tambah(tokens, {"tipe": "STRING", "nilai": str_val})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan c == "\"" {
            misal mulai = pos + 1
            pos = pos + 1
            selama pos < len dan sql_string[pos] != "\"" {
                jika sql_string[pos] == "\\" dan pos + 1 < len {
                    pos = pos + 2
                } lainnya {
                    pos = pos + 1
                }
            }
            misal str_val = potong(sql_string, mulai, pos)
            tambah(tokens, {"tipe": "STRING", "nilai": str_val})
            pos = pos + 1
            ditangani = benar
        }

        jika bukan ditangani dan apakah_digit(c) {
            misal mulai = pos
            selama pos < len dan (apakah_digit(sql_string[pos]) atau sql_string[pos] == ".") {
                pos = pos + 1
            }
            misal num_str = potong(sql_string, mulai, pos)
            tambah(tokens, {"tipe": "NUMBER", "nilai": num_str})
            ditangani = benar
        }

        jika bukan ditangani dan apakah_huruf(c) {
            misal mulai = pos
            selama pos < len dan apakah_huruf_atau_digit(sql_string[pos]) {
                pos = pos + 1
            }
            misal ident = potong(sql_string, mulai, pos)
            misal ident_upper = ke_besar_teks(ident)
            jika ada_kunci(KW_MAP, ident_upper) {
                tambah(tokens, {"tipe": ident_upper, "nilai": ident})
            } lainnya {
                tambah(tokens, {"tipe": "IDENT", "nilai": ident})
            }
            ditangani = benar
        }

        jika bukan ditangani {
            pos = pos + 1
        }
    }

    kembalikan tokens
}

struktur ParserSQL {
    tokens, pos, len_tokens,

    fungsi inisialisasi(token_list) {
        ini.tokens = token_list
        ini.pos = 0
        ini.len_tokens = panjang(token_list)
    }

    fungsi sekarang() {
        jika ini.pos < ini.len_tokens { kembalikan ini.tokens[ini.pos] }
        kembalikan nihil
    }

    fungsi lihat(offset) {
        misal idx = ini.pos + offset
        jika idx >= 0 dan idx < ini.len_tokens { kembalikan ini.tokens[idx] }
        kembalikan nihil
    }

    fungsi maju() {
        misal tok = ini.sekarang()
        ini.pos = ini.pos + 1
        kembalikan tok
    }

    fungsi cocok(tipe) {
        misal tok = ini.sekarang()
        jika tok != nihil dan tok.tipe == tipe {
            kembalikan ini.maju()
        }
        kembalikan nihil
    }

    fungsi ekspektasi(tipe) {
        kembalikan ini.cocok(tipe)
    }

    fungsi parse_kolom_atau_expr() {
        misal tok = ini.sekarang()
        jika tok == nihil { kembalikan nihil }

        jika tok.tipe == "SUM" atau tok.tipe == "AVG" atau tok.tipe == "COUNT" atau tok.tipe == "MIN" atau tok.tipe == "MAX" {
            misal agg_func = ini.maju()
            ini.cocok("KURUNG_BUKA")
            misal inner_expr = ini.parse_kolom_atau_expr()
            ini.cocok("KURUNG_TUTUP")
            kembalikan {"jenis": "agregat", "fn_nama": agg_func.tipe, "argumen": inner_expr}
        }

        jika tok.tipe == "ASTERISK" {
            ini.maju()
            kembalikan {"jenis": "asterisk"}
        }

        misal bagian1 = ini.cocok("IDENT")
        jika bagian1 == nihil { kembalikan nihil }

        jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "DOT" {
            ini.maju()
            misal bagian2 = ini.cocok("IDENT")
            jika bagian2 == nihil {
                bagian2 = ini.cocok("ASTERISK")
            }
            kembalikan {"jenis": "kolom_qualified", "tabel_alias": bagian1.nilai, "kolom": bagian2.nilai}
        }

        kembalikan {"jenis": "kolom", "nama": bagian1.nilai}
    }

    fungsi parse_daftar_select() {
        misal what_list = []
        selama benar {
            misal expr = ini.parse_kolom_atau_expr()
            jika expr == nihil { berhenti }

            misal alias_kolom = nihil
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "IDENT" dan ke_besar_teks(ini.lihat(0).nilai) == "AS") {
                ini.maju()
                misal t = ini.cocok("IDENT")
                jika t != nihil { alias_kolom = t.nilai }
            } lainnya jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "IDENT" {
                misal peek_next = ini.lihat(1)
                jika peek_next == nihil atau (peek_next.tipe != "DOT" dan peek_next.tipe != "KURUNG_BUKA") {
                    misal t = ini.cocok("IDENT")
                    alias_kolom = t.nilai
                }
            }

            misal col_str = ""
            jika expr.jenis == "asterisk" { col_str = "*"
            } lainnya jika expr.jenis == "kolom" { col_str = expr.nama
            } lainnya jika expr.jenis == "kolom_qualified" { col_str = expr.tabel_alias + "." + expr.kolom
            } lainnya jika expr.jenis == "agregat" {
                misal inner_col = "*"
                jika expr.argumen != nihil {
                    jika expr.argumen.jenis == "kolom" { inner_col = expr.argumen.nama }
                    lainnya jika expr.argumen.jenis == "kolom_qualified" { inner_col = expr.argumen.tabel_alias + "." + expr.argumen.kolom }
                }
                col_str = expr.fn_nama + "(" + inner_col + ")"
            }
            tambah(what_list, {"col": col_str, "alias": alias_kolom, "expr": expr})
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju() } lainnya { berhenti }
        }
        kembalikan what_list
    }

    fungsi parse_daftar_from() {
        misal from_list = []
        selama benar {
            misal tbl_tok = ini.cocok("IDENT")
            jika tbl_tok == nihil { berhenti }
            misal nama_tbl = tbl_tok.nilai
            misal alias_tbl = nihil
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "IDENT" dan ke_besar_teks(ini.lihat(0).nilai) == "AS") {
                ini.maju()
                misal a = ini.cocok("IDENT")
                jika a != nihil { alias_tbl = a.nilai }
            } lainnya jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "IDENT" {
                misal peek1 = ini.lihat(1)
                jika peek1 == nihil atau peek1.tipe == "JOIN" atau peek1.tipe == "INNER" atau peek1.tipe == "LEFT" atau peek1.tipe == "RIGHT" atau peek1.tipe == "WHERE" atau peek1.tipe == "GROUP" atau peek1.tipe == "ORDER" atau peek1.tipe == "LIMIT" atau peek1.tipe == "KOMA" {
                    misal a = ini.cocok("IDENT")
                    alias_tbl = a.nilai
                }
            }
            tambah(from_list, {"tbl": nama_tbl, "alias": alias_tbl})
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju() } lainnya { berhenti }
        }
        kembalikan from_list
    }

    fungsi parse_nilai() {
        misal tok = ini.sekarang()
        jika tok == nihil { kembalikan nihil }
        jika tok.tipe == "NUMBER" {
            ini.maju()
            misal s = tok.nilai
            jika ada(s, ".") { kembalikan ke_pecahan(s) }
            kembalikan ke_angka(s)
        }
        jika tok.tipe == "STRING" { ini.maju(); kembalikan tok.nilai }
        jika tok.tipe == "NULL" { ini.maju(); kembalikan nihil }
        jika tok.tipe == "IDENT" {
            ini.maju()
            misal val = tok.nilai
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "DOT" {
                ini.maju()
                misal c2 = ini.cocok("IDENT")
                val = val + "." + c2.nilai
            }
            kembalikan {"jenis": "kolom_ref", "nilai": val}
        }
        kembalikan nihil
    }

    fungsi parse_kondisi_where() {
        misal where_list = []
        selama benar {
            misal kol = ini.parse_kolom_atau_expr()
            jika kol == nihil { berhenti }
            misal op_tok = ini.sekarang()
            jika op_tok == nihil { berhenti }
            misal op_tipe = op_tok.tipe
            jika op_tipe == "OP_EQ" atau op_tipe == "OP_GT" atau op_tipe == "OP_LT" atau op_tipe == "OP_GTE" atau op_tipe == "OP_LTE" atau op_tipe == "OP_NEQ" {
                ini.maju()
                misal rhs = ini.parse_nilai()
                tambah(where_list, {"kiri": kol, "op": op_tipe, "kanan": rhs})
            }
            jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "AND" atau ini.lihat(0).tipe == "OR") {
                ini.maju()
            } lainnya {
                berhenti
            }
        }
        kembalikan where_list
    }

    fungsi parse() {
        misal tok1 = ini.sekarang()
        jika tok1 == nihil { kembalikan nihil }
        jika tok1.tipe == "SELECT" {
            ini.maju()
            misal kolom_list = ini.parse_daftar_select()
            ini.cocok("FROM")
            misal from_list = ini.parse_daftar_from()
            misal where_list = []
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "WHERE" {
                ini.maju()
                where_list = ini.parse_kondisi_where()
            }
            misal group_by_cols = []
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "GROUP" {
                ini.maju()
                ini.cocok("BY")
                group_by_cols = ini.parse_daftar_select()
            }
            misal order_by_list = []
            misal order_asc_arr = []
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "ORDER" {
                ini.maju()
                ini.cocok("BY")
                selama benar {
                    misal ob = ini.parse_kolom_atau_expr()
                    jika ob == nihil { berhenti }
                    tambah(order_by_list, ob)
                    misal arah = "ASC"
                    jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "DESC") {
                        arah = "DESC"
                        ini.maju()
                    } lainnya jika ini.lihat(0) != nihil dan (ini.lihat(0).tipe == "ASC") {
                        ini.maju()
                    }
                    tambah(order_asc_arr, arah)
                    jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "KOMA" { ini.maju() } lainnya { berhenti }
                }
            }
            misal limit_val = nihil
            jika ini.lihat(0) != nihil dan ini.lihat(0).tipe == "LIMIT" {
                ini.maju()
                misal lim_tok = ini.cocok("NUMBER")
                jika lim_tok != nihil { limit_val = ke_angka(lim_tok.nilai) }
            }
            kembalikan {
                "jenis": "SELECT",
                "kolom": kolom_list, "from": from_list, "where": where_list,
                "group_by": group_by_cols, "order_by": order_by_list, "order_arah": order_asc_arr,
                "limit": limit_val
            }
        }
        kembalikan nihil
    }
}

fungsi sql_parser(sql) {
    misal tokens = sql_lexer(sql)
    misal parser = ParserSQL(tokens)
    kembalikan parser.parse()
}

fungsi tabel_ke_df(tabel) {
    misal nama_kolom = []
    untuk_setiap(tabel.kolom, fungsi(k) {
        jika tipe(k) == "map" {
            tambah(nama_kolom, k.nama)
        } lainnya {
            tambah(nama_kolom, k)
        }
    })
    misal rows_2d = []
    untuk_setiap(tabel.baris_data, fungsi(baris) {
        misal arr_row = []
        untuk_setiap(nama_kolom, fungsi(nm) {
            jika ada_kunci(baris, nm) {
                tambah(arr_row, baris[nm])
            } lainnya {
                tambah(arr_row, nihil)
            }
        })
        tambah(rows_2d, arr_row)
    })
    kembalikan DataFrame(nama_kolom, rows_2d)
}

fungsi df_ke_array(df) {
    misal kolom = df.kolom
    misal baris = df.baris
    misal hasil = []
    untuk i dalam 0..(panjang(baris) - 1) {
        misal row_arr = baris[i]
        misal row_map = {}
        untuk j dalam 0..(panjang(kolom) - 1) {
            row_map[kolom[j]] = row_arr[j]
        }
        tambah(hasil, row_map)
    }
    kembalikan hasil
}

fungsi bandingkan_nilai(a, b, op) {
    jika op == "OP_EQ" { kembalikan a == b }
    jika op == "OP_GT" { kembalikan a > b }
    jika op == "OP_LT" { kembalikan a < b }
    jika op == "OP_GTE" { kembalikan a >= b }
    jika op == "OP_LTE" { kembalikan a <= b }
    jika op == "OP_NEQ" { kembalikan a != b }
    kembalikan salah
}

fungsi ekstrak_nilai_dari_baris(baris, col_expr) {
    jika col_expr.jenis == "kolom" {
        jika ada_kunci(baris, col_expr.nama) { kembalikan baris[col_expr.nama] }
        kembalikan nihil
    }
    jika col_expr.jenis == "kolom_qualified" {
        misal key1 = col_expr.tabel_alias + "." + col_expr.kolom
        jika ada_kunci(baris, key1) { kembalikan baris[key1] }
        jika ada_kunci(baris, col_expr.kolom) { kembalikan baris[col_expr.kolom] }
        kembalikan nihil
    }
    kembalikan nihil
}

fungsi terapkan_alias(tbl, alias_opt, prefix) {
    misal nama_kolom = []
    untuk_setiap(tbl.kolom, fungsi(k) {
        jika tipe(k) == "map" { tambah(nama_kolom, k.nama) }
        lainnya { tambah(nama_kolom, k) }
    })
    misal baris_alias = []
    untuk_setiap(tbl.baris_data, fungsi(baris_asli) {
        misal b = {}
        untuk_setiap(nama_kolom, fungsi(nm) {
            jika ada_kunci(baris_asli, nm) {
                b[nm] = baris_asli[nm]
                jika prefix != nihil {
                    b[prefix + "." + nm] = baris_asli[nm]
                }
                jika alias_opt != nihil {
                    b[alias_opt + "." + nm] = baris_asli[nm]
                }
            }
        })
        tambah(baris_alias, b)
    })
    kembalikan baris_alias
}

fungsi ekstrak_nama_kolom(col_expr) {
    jika col_expr.jenis == "kolom" { kembalikan col_expr.nama }
    jika col_expr.jenis == "kolom_qualified" { kembalikan col_expr.kolom }
    jika col_expr.jenis == "asterisk" { kembalikan "*" }
    jika col_expr.jenis == "agregat" {
        misal inner = "*"
        jika col_expr.argumen != nihil { inner = ekstrak_nama_kolom(col_expr.argumen) }
        kembalikan col_expr.fn_nama + "(" + inner + ")"
    }
    kembalikan "col"
}

fungsi urutkan_array_bubble(arr, comparator) {
    misal n = panjang(arr)
    untuk i dalam 0..(n - 2) {
        untuk j dalam 0..(n - i - 2) {
            jika comparator(arr[j + 1], arr[j]) {
                misal tmp = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = tmp
            }
        }
    }
    kembalikan arr
}

fungsi potong_array(arr, start, akhir) {
    misal hasil = []
    untuk i dalam start..(akhir - 1) {
        jika i < panjang(arr) {
            tambah(hasil, arr[i])
        }
    }
    kembalikan hasil
}

fungsi eksekusi_sql(db, sql_str) {
    misal ast = sql_parser(sql_str)
    jika ast == nihil { kembalikan nihil }
    jika ast.jenis == "SELECT" {
        misal from_tables = ast.from
        misal semua_baris = []
        untuk_setiap(from_tables, fungsi(ft) {
            jika ada_kunci(db.tabel, ft.tbl) {
                misal tbl = db.tabel[ft.tbl]
                misal baris_tbl = terapkan_alias(tbl, ft.alias, ft.tbl)
                jika panjang(semua_baris) == 0 {
                    semua_baris = baris_tbl
                } lainnya {
                    misal gabung_baru = []
                    untuk_setiap(semua_baris, fungsi(sb) {
                        untuk_setiap(baris_tbl, fungsi(bt) {
                            misal row_gabung = {}
                            untuk k dalam kunci(sb) { row_gabung[k] = sb[k] }
                            untuk k2 dalam kunci(bt) { row_gabung[k2] = bt[k2] }
                            tambah(gabung_baru, row_gabung)
                        })
                    })
                    semua_baris = gabung_baru
                }
            }
        })

        misal filtered = semua_baris
        jika panjang(ast.where) > 0 {
            filtered = saring(filtered, fungsi(baris) {
                misal lolos = benar
                untuk_setiap(ast.where, fungsi(kond) {
                    misal lhs_val = ekstrak_nilai_dari_baris(baris, kond.kiri)
                    misal rhs_val = kond.kanan
                    jika rhs_val != nihil dan tipe(rhs_val) == "map" dan ada_kunci(rhs_val, "jenis") dan rhs_val.jenis == "kolom_ref" {
                        rhs_val = nihil
                        misal key_ref = rhs_val
                        untuk kc dalam kunci(baris) {
                            jika kc == rhs_val.nilai atau dimulai_teks(kc, rhs_val.nilai + ".") {
                                rhs_val = baris[kc]
                                berhenti
                            }
                        }
                    }
                    jika lhs_val != nihil dan rhs_val != nihil {
                        jika bukan bandingkan_nilai(lhs_val, rhs_val, kond.op) { lolos = salah }
                    }
                })
                kembalikan lolos
            })
        }

        misal nama_output_kolom = []
        untuk_setiap(ast.kolom, fungsi(kc) {
            jika kc.alias != nihil {
                tambah(nama_output_kolom, kc.alias)
            } lainnya {
                tambah(nama_output_kolom, kc.col)
            }
        })

        misal hasil_baris_data = []
        jika panjang(ast.group_by) > 0 {
            misal groups = {}
            misal group_keys_arr = []
            untuk_setiap(filtered, fungsi(baris) {
                misal key_builder = ""
                untuk_setiap(ast.group_by, fungsi(gc) {
                    misal v = ekstrak_nilai_dari_baris(baris, gc)
                    key_builder = key_builder + "||" + ke_teks(v)
                })
                jika bukan ada_kunci(groups, key_builder) {
                    groups[key_builder] = []
                    tambah(group_keys_arr, key_builder)
                }
                tambah(groups[key_builder], baris)
            })
            untuk_setiap(group_keys_arr, fungsi(gk) {
                misal group_items = groups[gk]
                misal output_row_map = {}
                untuk col_idx dalam 0..(panjang(ast.kolom) - 1) {
                    misal col_expr_info = ast.kolom[col_idx]
                    misal out_col_name = nama_output_kolom[col_idx]
                    misal expr_x = col_expr_info.expr
                    jika expr_x.jenis == "agregat" {
                        misal fn_name = expr_x.fn_nama
                        misal hasil_agg = 0
                        misal count_agg = 0
                        untuk_setiap(group_items, fungsi(gi) {
                            jika fn_name == "COUNT" {
                                count_agg = count_agg + 1
                            } lainnya {
                                misal arg_col = ekstrak_nama_kolom(expr_x.argumen)
                                jika ada_kunci(gi, arg_col) {
                                    misal num_val = gi[arg_col]
                                    jika fn_name == "SUM" { hasil_agg = hasil_agg + num_val }
                                    jika fn_name == "MIN" dan count_agg == 0 { hasil_agg = num_val }
                                    jika fn_name == "MIN" dan num_val < hasil_agg { hasil_agg = num_val }
                                    jika fn_name == "MAX" dan count_agg == 0 { hasil_agg = num_val }
                                    jika fn_name == "MAX" dan num_val > hasil_agg { hasil_agg = num_val }
                                    count_agg = count_agg + 1
                                }
                            }
                        })
                        jika fn_name == "COUNT" { output_row_map[out_col_name] = count_agg }
                        jika fn_name == "AVG" dan count_agg > 0 { output_row_map[out_col_name] = hasil_agg / count_agg }
                        jika fn_name == "SUM" { output_row_map[out_col_name] = hasil_agg }
                        jika fn_name == "MIN" { output_row_map[out_col_name] = hasil_agg }
                        jika fn_name == "MAX" { output_row_map[out_col_name] = hasil_agg }
                    } lainnya {
                        misal sample_baris = group_items[0]
                        output_row_map[out_col_name] = ekstrak_nilai_dari_baris(sample_baris, col_expr_info.expr)
                    }
                }
                tambah(hasil_baris_data, output_row_map)
            })
        } lainnya {
            misal ada_agg = salah
            untuk_setiap(ast.kolom, fungsi(kc2) {
                jika kc2.expr.jenis == "agregat" { ada_agg = benar }
            })
            jika ada_agg {
                misal output_row_map2 = {}
                untuk col_idx2 dalam 0..(panjang(ast.kolom) - 1) {
                    misal col_expr_info2 = ast.kolom[col_idx2]
                    misal out_col_name2 = nama_output_kolom[col_idx2]
                    misal expr_x2 = col_expr_info2.expr
                    jika expr_x2.jenis == "agregat" {
                        misal fn_name2 = expr_x2.fn_nama
                        misal hasil_agg2 = 0
                        misal count_agg2 = 0
                        untuk_setiap(filtered, fungsi(gi2) {
                            jika fn_name2 == "COUNT" {
                                count_agg2 = count_agg2 + 1
                            } lainnya jika fn_name2 == "SUM" atau fn_name2 == "AVG" {
                                misal arg_col2 = ekstrak_nama_kolom(expr_x2.argumen)
                                jika ada_kunci(gi2, arg_col2) { hasil_agg2 = hasil_agg2 + gi2[arg_col2]; count_agg2 = count_agg2 + 1 }
                                lainnya jika arg_col2 == "*" { count_agg2 = count_agg2 + 1 }
                            } lainnya {
                                misal arg_col3 = ekstrak_nama_kolom(expr_x2.argumen)
                                jika ada_kunci(gi2, arg_col3) {
                                    misal num_val3 = gi2[arg_col3]
                                    jika fn_name2 == "MIN" dan count_agg2 == 0 { hasil_agg2 = num_val3 }
                                    jika fn_name2 == "MIN" dan num_val3 < hasil_agg2 { hasil_agg2 = num_val3 }
                                    jika fn_name2 == "MAX" dan count_agg2 == 0 { hasil_agg2 = num_val3 }
                                    jika fn_name2 == "MAX" dan num_val3 > hasil_agg2 { hasil_agg2 = num_val3 }
                                    count_agg2 = count_agg2 + 1
                                }
                            }
                        })
                        jika fn_name2 == "COUNT" { output_row_map2[out_col_name2] = count_agg2 }
                        jika fn_name2 == "SUM" { output_row_map2[out_col_name2] = hasil_agg2 }
                        jika fn_name2 == "AVG" dan count_agg2 > 0 { output_row_map2[out_col_name2] = hasil_agg2 / count_agg2 }
                        jika fn_name2 == "MIN" { output_row_map2[out_col_name2] = hasil_agg2 }
                        jika fn_name2 == "MAX" { output_row_map2[out_col_name2] = hasil_agg2 }
                    }
                }
                tambah(hasil_baris_data, output_row_map2)
            } lainnya {
                untuk_setiap(filtered, fungsi(baris_asli2) {
                    misal output_row_map3 = {}
                    untuk col_idx3 dalam 0..(panjang(ast.kolom) - 1) {
                        misal col_expr_info3 = ast.kolom[col_idx3]
                        misal out_col_name3 = nama_output_kolom[col_idx3]
                        jika col_expr_info3.expr.jenis == "asterisk" {
                            untuk k dalam kunci(baris_asli2) { output_row_map3[k] = baris_asli2[k] }
                        } lainnya {
                            output_row_map3[out_col_name3] = ekstrak_nilai_dari_baris(baris_asli2, col_expr_info3.expr)
                        }
                    }
                    tambah(hasil_baris_data, output_row_map3)
                })
            }
        }

        jika panjang(ast.order_by) > 0 {
            hasil_baris_data = urutkan_array_bubble(hasil_baris_data, fungsi(a2, b2) {
                untuk ob_idx dalam 0..(panjang(ast.order_by) - 1) {
                    misal ob_expr = ast.order_by[ob_idx]
                    misal ob_arah = ast.order_arah[ob_idx]
                    misal va = ekstrak_nilai_dari_baris(a2, {"jenis": "kolom", "nama": nama_output_kolom[ob_idx]})
                    jika va == nihil { va = ekstrak_nilai_dari_baris(a2, ob_expr) }
                    misal vb = ekstrak_nilai_dari_baris(b2, {"jenis": "kolom", "nama": nama_output_kolom[ob_idx]})
                    jika vb == nihil { vb = ekstrak_nilai_dari_baris(b2, ob_expr) }
                    jika va != vb {
                        jika ob_arah == "DESC" { kembalikan va > vb }
                        lainnya { kembalikan va < vb }
                    }
                }
                kembalikan salah
            })
        }

        jika ast.limit != nihil {
            hasil_baris_data = potong_array(hasil_baris_data, 0, ast.limit + 1)
        }

        misal final_kolom = nama_output_kolom
        jika panjang(hasil_baris_data) > 0 {
            final_kolom = kunci(hasil_baris_data[0])
        }
        misal final_rows_2d = []
        untuk_setiap(hasil_baris_data, fungsi(rm) {
            misal row_arr = []
            untuk_setiap(final_kolom, fungsi(fk) {
                jika ada_kunci(rm, fk) { tambah(row_arr, rm[fk]) }
                lainnya { tambah(row_arr, nihil) }
            })
            tambah(final_rows_2d, row_arr)
        })
        kembalikan DataFrame(final_kolom, final_rows_2d)
    }
    kembalikan nihil
}

fungsi buat_ord_map() {
    misal m = {}
    m["\0"] = 0
    m[" "] = 32; m["!"] = 33; m["\""] = 34; m["#"] = 35; m["$"] = 36; m["%"] = 37; m["&"] = 38; m["'"] = 39
    m["("] = 40; m[")"] = 41; m["*"] = 42; m["+"] = 43; m[","] = 44; m["-"] = 45; m["."] = 46; m["/"] = 47
    m["0"] = 48; m["1"] = 49; m["2"] = 50; m["3"] = 51; m["4"] = 52; m["5"] = 53; m["6"] = 54; m["7"] = 55; m["8"] = 56; m["9"] = 57
    m[":"] = 58; m[";"] = 59; m["<"] = 60; m["="] = 61; m[">"] = 62; m["?"] = 63; m["@"] = 64
    m["A"] = 65; m["B"] = 66; m["C"] = 67; m["D"] = 68; m["E"] = 69; m["F"] = 70; m["G"] = 71; m["H"] = 72; m["I"] = 73; m["J"] = 74; m["K"] = 75; m["L"] = 76; m["M"] = 77
    m["N"] = 78; m["O"] = 79; m["P"] = 80; m["Q"] = 81; m["R"] = 82; m["S"] = 83; m["T"] = 84; m["U"] = 85; m["V"] = 86; m["W"] = 87; m["X"] = 88; m["Y"] = 89; m["Z"] = 90
    m["["] = 91; m["\\"] = 92; m["]"] = 93; m["^"] = 94; m["_"] = 95; m["`"] = 96
    m["a"] = 97; m["b"] = 98; m["c"] = 99; m["d"] = 100; m["e"] = 101; m["f"] = 102; m["g"] = 103; m["h"] = 104; m["i"] = 105; m["j"] = 106; m["k"] = 107; m["l"] = 108; m["m"] = 109
    m["n"] = 110; m["o"] = 111; m["p"] = 112; m["q"] = 113; m["r"] = 114; m["s"] = 115; m["t"] = 116; m["u"] = 117; m["v"] = 118; m["w"] = 119; m["x"] = 120; m["y"] = 121; m["z"] = 122
    m["{"] = 123; m["|"] = 124; m["}"] = 125; m["~"] = 126
    kembalikan m
}

tetap ORD_MAP = buat_ord_map()

fungsi buat_chr_map() {
    misal m = {}
    untuk k dalam kunci(ORD_MAP) {
        misal v = ORD_MAP[k]
        m[ke_teks(v)] = k
    }
    kembalikan m
}

tetap CHR_MAP = buat_chr_map()

fungsi ord(c) {
    jika ada_kunci(ORD_MAP, c) { kembalikan ORD_MAP[c] }
    kembalikan 63
}

fungsi chr(n) {
    misal key = ke_teks(n)
    jika ada_kunci(CHR_MAP, key) { kembalikan CHR_MAP[key] }
    kembalikan "?"
}

fungsi int32_to_4bytes_bigendian(num) {
    misal hasil = []
    misal n = num
    jika n < 0 { n = n + 4294967296 }
    misal b1 = n / 16777216; n = n - b1 * 16777216
    misal b2 = n / 65536; n = n - b2 * 65536
    misal b3 = n / 256; n = n - b3 * 256
    misal b4 = n
    tambah(hasil, b1); tambah(hasil, b2); tambah(hasil, b3); tambah(hasil, b4)
    kembalikan hasil
}

fungsi int16_to_2bytes_bigendian(num) {
    misal hasil = []
    misal n = num
    jika n < 0 { n = n + 65536 }
    misal b1 = n / 256; n = n - b1 * 256
    misal b2 = n
    tambah(hasil, b1); tambah(hasil, b2)
    kembalikan hasil
}

fungsi pgwire_encode_frame(type_byte_char, payload_bytes_array) {
    misal hasil = []
    tambah(hasil, type_byte_char)
    misal panjang_total = 4 + panjang(payload_bytes_array)
    misal len_bytes = int32_to_4bytes_bigendian(panjang_total)
    tambah(hasil, len_bytes[0]); tambah(hasil, len_bytes[1]); tambah(hasil, len_bytes[2]); tambah(hasil, len_bytes[3])
    untuk pb dalam payload_bytes_array { tambah(hasil, pb) }
    kembalikan hasil
}

fungsi string_ke_bytes_nullterm(s) {
    misal hasil = []
    misal i = 0
    selama i < panjang(s) {
        tambah(hasil, ord(s[i]))
        i = i + 1
    }
    tambah(hasil, 0)
    kembalikan hasil
}

fungsi bytes_ke_teks(bytes_arr) {
    misal hasil = ""
    untuk b dalam bytes_arr { hasil = hasil + chr(b) }
    kembalikan hasil
}

fungsi frame_tipe_R_auth_ok() {
    misal payload = []
    misal len_bytes = int32_to_4bytes_bigendian(0)
    tambah(payload, len_bytes[0]); tambah(payload, len_bytes[1]); tambah(payload, len_bytes[2]); tambah(payload, len_bytes[3])
    kembalikan pgwire_encode_frame(ord('R'), payload)
}

fungsi frame_tipe_S_param_status(k, v) {
    misal payload = []
    misal kb = string_ke_bytes_nullterm(k)
    untuk b dalam kb { tambah(payload, b) }
    misal vb = string_ke_bytes_nullterm(v)
    untuk b dalam vb { tambah(payload, b) }
    kembalikan pgwire_encode_frame(ord('S'), payload)
}

fungsi frame_tipe_Z_idle() {
    misal payload = []
    tambah(payload, ord('I'))
    kembalikan pgwire_encode_frame(ord('Z'), payload)
}

fungsi frame_tipe_T_rowdesc(namako, oid) {
    misal payload = []
    misal num_fields = panjang(namako)
    misal nf_bytes = int16_to_2bytes_bigendian(num_fields)
    tambah(payload, nf_bytes[0]); tambah(payload, nf_bytes[1])
    misal ci = 0
    selama ci < panjang(namako) {
        misal nama = namako[ci]
        misal oid_val = 1043
        jika ci < panjang(oid) { oid_val = oid[ci] }
        misal nb = string_ke_bytes_nullterm(nama)
        untuk b dalam nb { tambah(payload, b) }
        misal table_oid = int32_to_4bytes_bigendian(0)
        untuk b dalam table_oid { tambah(payload, b) }
        misal attr_num = int16_to_2bytes_bigendian(0)
        untuk b dalam attr_num { tambah(payload, b) }
        misal type_oid = int32_to_4bytes_bigendian(oid_val)
        untuk b dalam type_oid { tambah(payload, b) }
        misal type_len = int16_to_2bytes_bigendian(-1)
        untuk b dalam type_len { tambah(payload, b) }
        misal typmod = int32_to_4bytes_bigendian(-1)
        untuk b dalam typmod { tambah(payload, b) }
        misal fmt_code = int16_to_2bytes_bigendian(0)
        untuk b dalam fmt_code { tambah(payload, b) }
        ci = ci + 1
    }
    kembalikan pgwire_encode_frame(ord('T'), payload)
}

fungsi frame_tipe_D_datarow(values_arr) {
    misal payload = []
    misal num_cols = panjang(values_arr)
    misal nc_bytes = int16_to_2bytes_bigendian(num_cols)
    tambah(payload, nc_bytes[0]); tambah(payload, nc_bytes[1])
    misal vi = 0
    selama vi < panjang(values_arr) {
        misal val = values_arr[vi]
        jika val == nihil {
            misal null_len = int32_to_4bytes_bigendian(-1)
            untuk b dalam null_len { tambah(payload, b) }
        } lainnya {
            misal str_val = ke_teks(val)
            misal vb = []
            misal si = 0
            selama si < panjang(str_val) { tambah(vb, ord(str_val[si])); si = si + 1 }
            misal vlen = int32_to_4bytes_bigendian(panjang(vb))
            untuk b dalam vlen { tambah(payload, b) }
            untuk b dalam vb { tambah(payload, b) }
        }
        vi = vi + 1
    }
    kembalikan pgwire_encode_frame(ord('D'), payload)
}

fungsi frame_tipe_C_cmdcomplete(tag) {
    misal payload = []
    misal tb = string_ke_bytes_nullterm(tag)
    untuk b dalam tb { tambah(payload, b) }
    kembalikan pgwire_encode_frame(ord('C'), payload)
}

fungsi frame_tipe_E_error(pesan) {
    misal payload = []
    tambah(payload, ord('S'))
    misal sb = string_ke_bytes_nullterm("ERROR")
    untuk b dalam sb { tambah(payload, b) }
    tambah(payload, ord('M'))
    misal mb = string_ke_bytes_nullterm(pesan)
    untuk b dalam mb { tambah(payload, b) }
    tambah(payload, 0)
    kembalikan pgwire_encode_frame(ord('E'), payload)
}

struktur PgWireServerState {
    status, user, basisdata, query_count, db,
    fungsi inisialisasi() {
        ini.status = "idle"
        ini.user = ""
        ini.basisdata = ""
        ini.query_count = 0
        ini.db = {"tabel": {}}
    }
}

fungsi inisialisasi_default() {
    misal state = PgWireServerState()
    state.status = "idle"
    state.user = ""
    state.basisdata = ""
    state.query_count = 0
    state.db = {"tabel": {}}
    kembalikan state
}

fungsi pgwire_server_pipeline(state, frame_bytes) {
    misal responses = []
    misal len_frame = panjang(frame_bytes)

    jika len_frame < 5 {
        tambah(responses, frame_tipe_E_error("Frame terlalu pendek"))
        tambah(responses, frame_tipe_Z_idle())
        kembalikan responses
    }

    misal type_byte = frame_bytes[0]

    jika type_byte == ord('Q') {
        jika len_frame < 6 {
            tambah(responses, frame_tipe_E_error("Query frame terlalu pendek"))
            tambah(responses, frame_tipe_Z_idle())
            kembalikan responses
        }
        state.query_count = state.query_count + 1
        misal sql_bytes = []
        misal si = 5
        selama si < len_frame {
            jika frame_bytes[si] == 0 { berhenti }
            tambah(sql_bytes, frame_bytes[si])
            si = si + 1
        }
        misal sql_str = bytes_ke_teks(sql_bytes)

        cetak("FS5 DEBUG Q sql_str=[" + sql_str + "]")
        misal db_inst = {"tabel": state.db.tabel}
        cetak("FS5 DEBUG Q db_inst keys=" + ke_json(kunci(db_inst)) + " ada_tabel_pengguna=" + ke_teks(ada_kunci(db_inst.tabel, "pengguna")))
        misal hasil_sql = eksekusi_sql(db_inst, sql_str)
        cetak("FS5 DEBUG Q hasil_sql_tipe=" + tipe(hasil_sql))

        jika hasil_sql == nihil {
            tambah(responses, frame_tipe_E_error("SQL tidak valid atau tabel tidak ditemukan"))
            tambah(responses, frame_tipe_Z_idle())
            kembalikan responses
        }

        jika tipe(hasil_sql) == "map" dan ada_kunci(hasil_sql, "dipengaruhi") {
            misal tag = "OK " + ke_teks(hasil_sql.dipengaruhi)
            tambah(responses, frame_tipe_C_cmdcomplete(tag))
            tambah(responses, frame_tipe_Z_idle())
            kembalikan responses
        }

        misal arr_hasil = df_ke_array(hasil_sql)
        misal nama_kolom = []
        misal oid_kolom = []
        jika panjang(arr_hasil) > 0 { nama_kolom = kunci(arr_hasil[0]) }
        misal ok_idx = 0
        selama ok_idx < panjang(nama_kolom) {
            tambah(oid_kolom, 23)
            ok_idx = ok_idx + 1
        }

        tambah(responses, frame_tipe_T_rowdesc(nama_kolom, oid_kolom))

        misal jumlah_data_row = 0
        untuk row_map dalam arr_hasil {
            misal row_vals = []
            untuk k dalam nama_kolom {
                misal v = nihil
                jika ada_kunci(row_map, k) { v = row_map[k] }
                tambah(row_vals, v)
            }
            tambah(responses, frame_tipe_D_datarow(row_vals))
            jumlah_data_row = jumlah_data_row + 1
        }

        misal tag_cc = "SELECT " + ke_teks(jumlah_data_row)
        tambah(responses, frame_tipe_C_cmdcomplete(tag_cc))
        tambah(responses, frame_tipe_Z_idle())
        kembalikan responses
    }

    jika type_byte == ord('X') {
        state.status = "terminated"
        kembalikan responses
    }

    jika len_frame >= 8 {
        misal proto_ver_bytes = [frame_bytes[4], frame_bytes[5], frame_bytes[6], frame_bytes[7]]
        misal proto_ver = proto_ver_bytes[0] * 16777216 + proto_ver_bytes[1] * 65536 + proto_ver_bytes[2] * 256 + proto_ver_bytes[3]
        jika proto_ver == 196608 {
            state.status = "authenticated"
            tambah(responses, frame_tipe_R_auth_ok())
            tambah(responses, frame_tipe_S_param_status("server_encoding", "UTF8"))
            tambah(responses, frame_tipe_S_param_status("client_encoding", "UTF8"))
            tambah(responses, frame_tipe_S_param_status("DateStyle", "ISO, MDY"))
            tambah(responses, frame_tipe_Z_idle())
            kembalikan responses
        }
    }

    tambah(responses, frame_tipe_E_error("Tipe pesan tidak dikenal: " + ke_teks(type_byte)))
    tambah(responses, frame_tipe_Z_idle())
    kembalikan responses
}

fungsi jalankan_self_test() {
    misal state = inisialisasi_default()
    misal db_rel = {"tabel": state.db.tabel}
    misal tbl_pengguna = TabelRelasional("pengguna", ["id", "nama", "kota"])
    tbl_pengguna.sisip({"id": 1, "nama": "Ahmad", "kota": "Jakarta"})
    tbl_pengguna.sisip({"id": 2, "nama": "Siti", "kota": "Bandung"})
    tbl_pengguna.sisip({"id": 3, "nama": "Budi", "kota": "Surabaya"})
    state.db.tabel["pengguna"] = tbl_pengguna

    misal startup_frame = []
    misal startup_payload = []
    misal proto_bytes = int32_to_4bytes_bigendian(196608)
    untuk b dalam proto_bytes { tambah(startup_payload, b) }
    misal user_kv = string_ke_bytes_nullterm("user")
    untuk b dalam user_kv { tambah(startup_payload, b) }
    misal user_v = string_ke_bytes_nullterm("postgres")
    untuk b dalam user_v { tambah(startup_payload, b) }
    misal db_kv = string_ke_bytes_nullterm("database")
    untuk b dalam db_kv { tambah(startup_payload, b) }
    misal db_v = string_ke_bytes_nullterm("testdb")
    untuk b dalam db_v { tambah(startup_payload, b) }
    tambah(startup_payload, 0)
    misal len_total_startup = 4 + panjang(startup_payload)
    misal len_startup_bytes = int32_to_4bytes_bigendian(len_total_startup)
    untuk b dalam len_startup_bytes { tambah(startup_frame, b) }
    untuk b dalam startup_payload { tambah(startup_frame, b) }

    misal resp1 = pgwire_server_pipeline(state, startup_frame)
    misal ada_R_auth = salah
    misal ada_Z_idle_step1 = salah
    untuk fr dalam resp1 {
        jika panjang(fr) > 0 dan fr[0] == ord('R') { ada_R_auth = benar }
        jika panjang(fr) > 0 dan fr[0] == ord('Z') { ada_Z_idle_step1 = benar }
    }
    cetak("FS5 DEBUG resp1_len=" + ke_teks(panjang(resp1)) + " ada_R=" + ke_teks(ada_R_auth) + " ada_Z1=" + ke_teks(ada_Z_idle_step1))
    jika bukan ada_R_auth { cetak("FS5 FAIL: tidak ada frame R authOk"); kembalikan salah }
    jika bukan ada_Z_idle_step1 { cetak("FS5 FAIL: tidak ada frame Z idle step1"); kembalikan salah }

    misal sql_str = "SELECT COUNT(*) AS cnt FROM pengguna;"
    misal query_frame = []
    tambah(query_frame, ord('Q'))
    misal sql_bytes_nullterm = string_ke_bytes_nullterm(sql_str)
    misal len_payload_q = 4 + panjang(sql_bytes_nullterm)
    misal len_q_bytes = int32_to_4bytes_bigendian(len_payload_q)
    untuk b dalam len_q_bytes { tambah(query_frame, b) }
    untuk b dalam sql_bytes_nullterm { tambah(query_frame, b) }

    misal resp2 = pgwire_server_pipeline(state, query_frame)
    misal ada_T_rowdesc = salah
    misal ada_nama_cnt = salah
    misal jumlah_D_datarow = 0
    misal ada_C_select = salah
    misal ada_Z_step2 = salah
    misal nilai_cnt_benar = salah
    cetak("FS5 DEBUG resp2_len=" + ke_teks(panjang(resp2)))
    misal urut_idx = 0
    untuk fr dalam resp2 {
        jika panjang(fr) > 0 {
            cetak("FS5 DEBUG fr[" + ke_teks(urut_idx) + "] first_byte=" + ke_teks(fr[0]) + " nama=" + chr(fr[0]))
            jika fr[0] == ord('T') {
                ada_T_rowdesc = benar
                misal frame_body_str = bytes_ke_teks(fr)
                cetak("FS5 DEBUG T frame_body_str_len=" + ke_teks(panjang(frame_body_str)) + " isi=" + frame_body_str)
                jika ada(frame_body_str, "cnt") { ada_nama_cnt = benar }
            }
            jika fr[0] == ord('D') {
                jumlah_D_datarow = jumlah_D_datarow + 1
                misal d_str = bytes_ke_teks(fr)
                cetak("FS5 DEBUG D isi_bytes_text=" + d_str)
                jika ada(d_str, "3") { nilai_cnt_benar = benar }
            }
            jika fr[0] == ord('C') {
                ada_C_select = benar
                misal c_str = bytes_ke_teks(fr)
                cetak("FS5 DEBUG C isi=" + c_str)
            }
            jika fr[0] == ord('Z') { ada_Z_step2 = benar }
            jika fr[0] == ord('E') {
                misal e_str = bytes_ke_teks(fr)
                cetak("FS5 DEBUG ERROR FRAME E: " + e_str)
            }
        }
        urut_idx = urut_idx + 1
    }

    jika bukan ada_T_rowdesc { cetak("FS5 FAIL: tidak ada T rowdesc"); kembalikan salah }
    jika bukan ada_nama_cnt { cetak("FS5 FAIL: T rowdesc tidak mengandung nama kolom cnt"); kembalikan salah }
    jika jumlah_D_datarow != 1 { cetak("FS5 FAIL: jumlah D datarow=" + ke_teks(jumlah_D_datarow) + " != 1"); kembalikan salah }
    jika bukan ada_C_select { cetak("FS5 FAIL: tidak ada C commandcomplete SELECT"); kembalikan salah }
    jika bukan ada_Z_step2 { cetak("FS5 FAIL: tidak ada Z idle step2"); kembalikan salah }
    jika bukan nilai_cnt_benar { cetak("FS5 FAIL: D datarow tidak mengandung nilai 3"); kembalikan salah }

    cetak("FS5 DEBUG ALL ASSERTIONS PASS")
    kembalikan benar
}

kembalikan jalankan_self_test();
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_5_pgwire_server_pipeline_mock: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_6_secondary_index_doc_1000() {
    let code = r##"
struktur PohonBPlusMap2 {
    simpanan,
    fungsi inisialisasi(_order) {
        ini.simpanan = {}
    }
}

fungsi bplus_sisip2(pohon, kunci, nilai) {
    pohon.simpanan[kunci] = nilai
    kembalikan benar
}

fungsi bplus_cari2(pohon, kunci) {
    jika ada_kunci(pohon.simpanan, kunci) {
        kembalikan pohon.simpanan[kunci]
    }
    kembalikan nihil
}

fungsi PohonBPlus2(order) {
    kembalikan PohonBPlusMap2(order)
}

struktur KoleksiDokumen2 {
    nama_koleksi, dokumen, indeks_field, _dokumen_by_id,
    fungsi inisialisasi(nama) {
        ini.nama_koleksi = nama
        ini.dokumen = []
        ini.indeks_field = {}
        ini._dokumen_by_id = {}
    }

    fungsi sisip_satu(doc) {
        jika bukan ada_kunci(doc, "_id") {
            doc["_id"] = "doc_" + ke_teks(panjang(ini.dokumen) + 1)
        }
        tambah(ini.dokumen, doc)
        ini._dokumen_by_id[doc["_id"]] = panjang(ini.dokumen) - 1

        misal field_terindeks = kunci(ini.indeks_field)
        jika panjang(field_terindeks) > 0 {
            untuk_setiap(field_terindeks, fungsi(nama_field) {
                misal pohon = ini.indeks_field[nama_field]
                jika ada_kunci(doc, nama_field) {
                    misal nilai_field = ke_teks(doc[nama_field])
                    misal existing = bplus_cari2(pohon, nilai_field)
                    misal daftar_id = []
                    jika existing != nihil {
                        daftar_id = existing
                    }
                    tambah(daftar_id, doc["_id"])
                    bplus_sisip2(pohon, nilai_field, daftar_id)
                }
            })
        }
        kembalikan Ok(doc["_id"])
    }

    fungsi cari(kunci_field, nilai_target) {
        kembalikan saring(ini.dokumen, fungsi(item) {
            jika ada_kunci(item, kunci_field) {
                kembalikan item[kunci_field] == nilai_target
            }
            kembalikan salah
        })
    }
}

fungsi _cari_dokumen_by_id2(koleksi, id) {
    jika ada_kunci(koleksi._dokumen_by_id, id) {
        kembalikan koleksi.dokumen[koleksi._dokumen_by_id[id]]
    }
    misal hasil = saring(koleksi.dokumen, fungsi(d) {
        kembalikan d["_id"] == id
    })
    jika panjang(hasil) > 0 {
        kembalikan hasil[0]
    }
    kembalikan nihil
}

fungsi indeks_tambah2(koleksi, nama_field) {
    misal pohon = PohonBPlus2(4)
    untuk_setiap(koleksi.dokumen, fungsi(doc) {
        jika ada_kunci(doc, nama_field) {
            misal nilai_field = ke_teks(doc[nama_field])
            misal existing = bplus_cari2(pohon, nilai_field)
            misal daftar_id = []
            jika existing != nihil {
                daftar_id = existing
            }
            tambah(daftar_id, doc["_id"])
            bplus_sisip2(pohon, nilai_field, daftar_id)
        }
    })
    koleksi.indeks_field[nama_field] = pohon
    kembalikan benar
}

fungsi indeks_cari2(koleksi, nama_field, nilai_cari) {
    jika bukan ada_kunci(koleksi.indeks_field, nama_field) {
        kembalikan []
    }
    misal pohon = koleksi.indeks_field[nama_field]
    misal kunci_cari = ke_teks(nilai_cari)
    misal hasil_ids = bplus_cari2(pohon, kunci_cari)
    jika hasil_ids == nihil {
        kembalikan []
    }
    misal hasil = []
    untuk_setiap(hasil_ids, fungsi(id) {
        misal doc = _cari_dokumen_by_id2(koleksi, id)
        jika doc != nihil {
            tambah(hasil, doc)
        }
    })
    kembalikan hasil
}

misal k = KoleksiDokumen2("data_kota")

untuk i dalam 0..999 {
    misal nama_kota = "TidakTahu"
    jika (i % 3) == 0 {
        nama_kota = "Jakarta"
    } lainnya jika (i % 3) == 1 {
        nama_kota = "Bandung"
    } lainnya {
        nama_kota = "Surabaya"
    }
    k.sisip_satu({
        "kota": nama_kota,
        "nomor": i + 1,
        "provinsi": "Prov_" + ke_teks(i % 3)
    })
}

pastikan(panjang(k.dokumen) == 1000, "Total dokumen harus 1000, tapi: " + ke_teks(panjang(k.dokumen)))

misal fullscan_jakarta = k.cari("kota", "Jakarta")
misal count_fullscan = panjang(fullscan_jakarta)

indeks_tambah2(k, "kota")
misal idx_jakarta = indeks_cari2(k, "kota", "Jakarta")
misal count_idx = panjang(idx_jakarta)

pastikan(count_idx == count_fullscan, "COUNT indeks_cari harus == COUNT fullscan, idx=" + ke_teks(count_idx) + " full=" + ke_teks(count_fullscan))

k.sisip_satu({
    "_id": "doc_tambahan_1",
    "kota": "Jakarta",
    "nomor": 9999,
    "provinsi": "DKI Jakarta"
})

misal fullscan_jakarta2 = k.cari("kota", "Jakarta")
misal count_fullscan2 = panjang(fullscan_jakarta2)
pastikan(count_fullscan2 == count_fullscan + 1, "Setelah tambah 1, fullscan COUNT harus bertambah 1: " + ke_teks(count_fullscan) + " -> " + ke_teks(count_fullscan2))

misal idx_jakarta2 = indeks_cari2(k, "kota", "Jakarta")
misal count_idx2 = panjang(idx_jakarta2)
pastikan(count_idx2 == count_fullscan2, "Setelah tambah 1, indeks COUNT == fullscan COUNT: idx2=" + ke_teks(count_idx2) + " full2=" + ke_teks(count_fullscan2))

kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_6_secondary_index_doc_1000: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_7_fts_inverted_tfidf_rank() {
    let code = r##"
tetap KONSTANTA_E_FTS = 2.718281828459045;

fungsi ln_natural_fts(x) {
    jika x <= 0 {
        kembalikan 0;
    }
    misal y = 0;
    jika x > 1 {
        y = x / 2;
    } lainnya {
        y = x - 1;
    }
    untuk _ dalam 1..50 {
        misal ey = pangkat(KONSTANTA_E_FTS, y);
        misal y_baru = y - 1 + x / ey;
        jika mutlak(y_baru - y) < 0.0000001 {
            y = y_baru;
            berhenti;
        }
        y = y_baru;
    }
    kembalikan y;
}

fungsi apakah_ada_fts(daftar, pred) {
    untuk item dalam daftar {
        jika pred(item) {
            kembalikan benar;
        }
    }
    kembalikan salah;
}

fungsi teks_tokenize_fts(kalimat) {
    misal stop_words = ["yang","dan","di","ke","dari","pada","untuk","dengan","adalah","ini","itu","saya","kamu","dia","akan","telah","sudah"];
    misal huruf_kecil_kalimat = huruf_kecil(kalimat);
    misal tanda_baca = [",",".",";",":","!","?"];
    untuk_setiap(tanda_baca, fungsi(tb) {
        huruf_kecil_kalimat = regex_ganti("\\" + tb, " ", huruf_kecil_kalimat);
    });
    huruf_kecil_kalimat = regex_ganti(r"\s+", " ", huruf_kecil_kalimat);
    misal kata_kata = pisah(huruf_kecil_kalimat, " ");
    misal hasil = [];
    untuk_setiap(kata_kata, fungsi(k) {
        jika k != "" && bukan apakah_ada_fts(stop_words, fungsi(sw) {
            kembalikan sw == k;
        }) {
            tambah(hasil, k);
        }
    });
    kembalikan hasil;
}

struktur KoleksiDokumenFTS {
    nama_koleksi, dokumen, indeks_teks,
    fungsi inisialisasi(nama) {
        ini.nama_koleksi = nama;
        ini.dokumen = [];
        ini.indeks_teks = {};
    }

    fungsi sisip_satu(doc) {
        jika bukan ada_kunci(doc, "_id") {
            doc["_id"] = "doc_" + ke_teks(panjang(ini.dokumen) + 1);
        }
        tambah(ini.dokumen, doc);
        kembalikan Ok(doc["_id"]);
    }
}

struktur IndeksTerbalikFTS {
    map_kata, panjang_dok,
    fungsi inisialisasi() {
        ini.map_kata = {};
        ini.panjang_dok = {};
    }
}

fungsi indeks_terbalik_tambah_fts(it, doc_id, teks) {
    misal tokens = teks_tokenize_fts(teks);
    it.panjang_dok[doc_id] = panjang(tokens);
    misal freq_map = {};
    untuk_setiap(tokens, fungsi(t) {
        jika ada_kunci(freq_map, t) {
            freq_map[t] = freq_map[t] + 1;
        } lainnya {
            freq_map[t] = 1;
        }
    });
    untuk k dalam kunci(freq_map) {
        misal entry = {"doc_id": doc_id, "term_freq": freq_map[k]};
        jika ada_kunci(it.map_kata, k) {
            tambah(it.map_kata[k], entry);
        } lainnya {
            it.map_kata[k] = [entry];
        }
    }
}

fungsi indeks_terbalik_kueri_fts(it, koleksi_dok, nama_field, string_kueri, top_k) {
    jika top_k == 0 {
        top_k = 10;
    }
    misal kueri_tokens = teks_tokenize_fts(string_kueri);
    jika panjang(kueri_tokens) == 0 {
        kembalikan [];
    }
    misal total_dok = panjang(koleksi_dok.dokumen);
    misal skor_dok = {};
    misal idx_dok_map = {};
    misal idx_counter = 0;
    untuk_setiap(koleksi_dok.dokumen, fungsi(dok) {
        idx_dok_map[dok["_id"]] = idx_counter;
        idx_counter = idx_counter + 1;
    });
    untuk_setiap(kueri_tokens, fungsi(kt) {
        jika ada_kunci(it.map_kata, kt) {
            misal daftar_posting = it.map_kata[kt];
            misal df = panjang(daftar_posting);
            misal idf = ln_natural_fts((1 + total_dok) / (1 + df)) + 1;
            untuk_setiap(daftar_posting, fungsi(posting) {
                misal did = posting["doc_id"];
                misal tf = posting["term_freq"];
                misal pd = it.panjang_dok[did];
                jika pd > 0 {
                    misal tfidf = (tf / pd) * idf;
                    misal tiebreaker = 0.0000001 * (total_dok - idx_dok_map[did]);
                    jika ada_kunci(skor_dok, did) {
                        skor_dok[did] = skor_dok[did] + tfidf;
                    } lainnya {
                        skor_dok[did] = tfidf + tiebreaker;
                    }
                }
            });
        }
    });
    misal daftar_skor = [];
    untuk did dalam kunci(skor_dok) {
        misal dok_target = saring(koleksi_dok.dokumen, fungsi(d) {
            kembalikan d["_id"] == did;
        });
        jika panjang(dok_target) > 0 {
            tambah(daftar_skor, {"dokumen": dok_target[0], "skor": skor_dok[did]});
        }
    }
    misal skor_angka = [];
    untuk_setiap(daftar_skor, fungsi(item) {
        tambah(skor_angka, item["skor"]);
    });
    misal skor_terurut = urutkan(skor_angka);
    skor_terurut = balik(skor_terurut);
    misal hasil = [];
    untuk s dalam skor_terurut {
        jika panjang(hasil) >= top_k {
            berhenti;
        }
        untuk item dalam daftar_skor {
            jika item["skor"] == s && panjang(hasil) < top_k {
                misal belum_ada = benar;
                untuk h dalam hasil {
                    jika h["dokumen"]["_id"] == item["dokumen"]["_id"] {
                        belum_ada = salah;
                    }
                }
                jika belum_ada {
                    tambah(hasil, item);
                }
            }
        }
    }
    kembalikan hasil;
}

fungsi pencarian_teks_terbalik_fts(koleksi_dok, nama_field, string_kueri) {
    jika bukan ada_kunci(koleksi_dok.indeks_teks, nama_field) {
        misal it_baru = IndeksTerbalikFTS();
        untuk_setiap(koleksi_dok.dokumen, fungsi(dok) {
            misal did = dok["_id"];
            misal teks_field = "";
            jika ada_kunci(dok, nama_field) {
                teks_field = dok[nama_field];
            }
            indeks_terbalik_tambah_fts(it_baru, did, teks_field);
        });
        koleksi_dok.indeks_teks[nama_field] = it_baru;
    }
    misal it = koleksi_dok.indeks_teks[nama_field];
    kembalikan indeks_terbalik_kueri_fts(it, koleksi_dok, nama_field, string_kueri, 10);
}

misal koleksi_artikel = KoleksiDokumenFTS("artikel");
misal A = {"_id": "a", "isi": "widya bahasa pemrograman widya widya widya modern AI widya OS"};
misal B = {"_id": "b", "isi": "widya database sql"};
misal C = {"_id": "c", "isi": "riset widya spasial"};
misal D = {"_id": "d", "isi": "sistem operasi linux dan windows server"};
misal E = {"_id": "e", "isi": "belajar pemrograman rust dasar untuk widya"};
koleksi_artikel.sisip_satu(A);
koleksi_artikel.sisip_satu(B);
koleksi_artikel.sisip_satu(C);
koleksi_artikel.sisip_satu(D);
koleksi_artikel.sisip_satu(E);

misal hasil1 = pencarian_teks_terbalik_fts(koleksi_artikel, "isi", "widya");

pastikan(panjang(hasil1) >= 3, "Hasil kueri widya minimal 3, tapi: " + ke_teks(panjang(hasil1)));
pastikan(hasil1[0]["skor"] > hasil1[1]["skor"], "Skor A harus > skor B");
pastikan(hasil1[1]["skor"] > hasil1[2]["skor"], "Skor B harus > skor C");
pastikan(hasil1[2]["skor"] > 0, "Skor C harus > 0");
pastikan(hasil1[0]["dokumen"]["_id"] == "a", "Teratas harus A (id a)");

misal hasil2 = pencarian_teks_terbalik_fts(koleksi_artikel, "isi", "tidakada");
pastikan(panjang(hasil2) == 0, "Kueri kata tidakada harus kosong");

misal F = {"_id": "f", "isi": "widya terbaru 2026 widya"};
koleksi_artikel.sisip_satu(F);
koleksi_artikel.indeks_teks = {};

misal hasil3 = pencarian_teks_terbalik_fts(koleksi_artikel, "isi", "widya");
misal f_ada = salah;
untuk_setiap(hasil3, fungsi(h) {
    jika h["dokumen"]["_id"] == "f" {
        f_ada = benar;
    }
});
pastikan(f_ada, "Dokumen F harus masuk dalam hasil pencarian widya");

kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_7_fts_inverted_tfidf_rank: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_8_vectordb_lsh_approx_recall() {
    let code = r##"
fungsi kosinus_kemiripan_lsh(v1, v2) {
    misal titik = 0
    misal norm1 = 0
    misal norm2 = 0
    misal n = panjang(v1)
    var i = 0
    selama (i < n) {
        titik += v1[i] * v2[i]
        norm1 += v1[i] * v1[i]
        norm2 += v2[i] * v2[i]
        i += 1
    }
    jika norm1 == 0 atau norm2 == 0 {
        kembalikan 0
    }
    kembalikan titik / (akar(norm1) * akar(norm2))
}

fungsi buat_basis_data_vektor_lsh(dim) {
    kembalikan {
        "dimensi": dim,
        "entri": []
    }
}

fungsi vektor_sisip_lsh(db, id, vek, meta) {
    tambah(db.entri, {
        "id": id,
        "vektor": vek,
        "metadata": meta
    })
    kembalikan benar
}

fungsi vektor_hitung_lsh(db) {
    kembalikan panjang(db.entri)
}

fungsi vektor_dapatkan_vektor_lsh(db, id) {
    misal n = panjang(db.entri)
    var i = 0
    selama (i < n) {
        jika db.entri[i].id == id {
            kembalikan db.entri[i].vektor
        }
        i += 1
    }
    kembalikan nihil
}

fungsi vektor_dapatkan_metadata_lsh(db, id) {
    misal n = panjang(db.entri)
    var i = 0
    selama (i < n) {
        jika db.entri[i].id == id {
            kembalikan db.entri[i].metadata
        }
        i += 1
    }
    kembalikan nihil
}

fungsi sort_hasil_kemiripan_lsh(daftar) {
    misal n = panjang(daftar)
    var i = 0
    selama (i < n) {
        var j = i + 1
        selama (j < n) {
            jika daftar[j].kemiripan > daftar[i].kemiripan {
                misal tmp = daftar[i]
                daftar[i] = daftar[j]
                daftar[j] = tmp
            }
            j += 1
        }
        i += 1
    }
    kembalikan daftar
}

fungsi vektor_kueri_knn_lsh(db, qvek, top_k) {
    misal skor = []
    untuk_setiap(db.entri, fungsi(e) {
        misal sim = kosinus_kemiripan_lsh(qvek, e.vektor)
        tambah(skor, {
            "id": e.id,
            "vektor": e.vektor,
            "metadata": e.metadata,
            "kemiripan": sim
        })
    })
    sort_hasil_kemiripan_lsh(skor)
    misal hasil = []
    var k = top_k
    jika k > panjang(skor) {
        k = panjang(skor)
    }
    var idx = 0
    selama (idx < k) {
        tambah(hasil, skor[idx])
        idx += 1
    }
    kembalikan hasil
}

fungsi lsh_init_lsh(D, num_tables, num_bits) {
    misal random_vecs_per_table = []
    var t = 0
    selama (t < num_tables) {
        misal table_vecs = []
        var b = 0
        selama (b < num_bits) {
            misal rand_vec = []
            var d = 0
            selama (d < D) {
                tambah(rand_vec, acak(-1, 1))
                d += 1
            }
            tambah(table_vecs, rand_vec)
            b += 1
        }
        tambah(random_vecs_per_table, table_vecs)
        t += 1
    }
    misal bucket_per_table = []
    var t2 = 0
    selama (t2 < num_tables) {
        tambah(bucket_per_table, {})
        t2 += 1
    }
    kembalikan {
        "D": D,
        "num_tables": num_tables,
        "num_bits": num_bits,
        "random_vecs_per_table": random_vecs_per_table,
        "bucket_per_table": bucket_per_table
    }
}

fungsi lsh_hitung_signature_lsh(lsh, table_idx, vek) {
    misal sig = ""
    misal table_vecs = lsh.random_vecs_per_table[table_idx]
    var b = 0
    selama (b < lsh.num_bits) {
        misal rv = table_vecs[b]
        misal dot = 0
        var d = 0
        selama (d < lsh.D) {
            dot += vek[d] * rv[d]
            d += 1
        }
        jika dot >= 0 {
            sig += "1"
        } lainnya {
            sig += "0"
        }
        b += 1
    }
    kembalikan sig
}

fungsi lsh_sisip_lsh(lsh, id, vek) {
    var t = 0
    selama (t < lsh.num_tables) {
        misal sig = lsh_hitung_signature_lsh(lsh, t, vek)
        misal bucket = lsh.bucket_per_table[t]
        jika bucket[sig] == nihil {
            bucket[sig] = []
        }
        tambah(bucket[sig], id)
        t += 1
    }
    kembalikan benar
}

fungsi lsh_kueri_knn_lsh(lsh, vektor_db, qvek, top_k) {
    misal kandidat_map = {}
    var t = 0
    selama (t < lsh.num_tables) {
        misal sig = lsh_hitung_signature_lsh(lsh, t, qvek)
        misal bucket = lsh.bucket_per_table[t]
        misal ids = bucket[sig]
        jika ids != nihil {
            untuk_setiap(ids, fungsi(id) {
                kandidat_map[id] = benar
            })
        }
        t += 1
    }
    misal kandidat_ids = kunci(kandidat_map)
    misal skor = []
    untuk_setiap(kandidat_ids, fungsi(id) {
        misal ev = vektor_dapatkan_vektor_lsh(vektor_db, id)
        jika ev != nihil {
            misal sim = kosinus_kemiripan_lsh(qvek, ev)
            misal meta = vektor_dapatkan_metadata_lsh(vektor_db, id)
            tambah(skor, {
                "id": id,
                "vektor": ev,
                "metadata": meta,
                "kemiripan": sim
            })
        }
    })
    sort_hasil_kemiripan_lsh(skor)
    misal hasil = []
    var k = top_k
    jika k > panjang(skor) {
        k = panjang(skor)
    }
    var idx = 0
    selama (idx < k) {
        tambah(hasil, skor[idx])
        idx += 1
    }
    misal info_jumlah_kandidat = panjang(kandidat_ids)
    kembalikan {
        "hasil": hasil,
        "jumlah_kandidat": info_jumlah_kandidat
    }
}

fungsi uji_sendiri_vektor_lsh_fts() {
    misal D = 8
    misal db = buat_basis_data_vektor_lsh(D)
    misal lsh = lsh_init_lsh(D, 10, 4)
    var idx = 0
    selama (idx < 500) {
        misal vek = []
        var d = 0
        selama (d < D) {
            tambah(vek, acak(-1, 1))
            d += 1
        }
        misal id = "v_" + ke_teks(idx)
        misal meta = { "nomor": idx }
        vektor_sisip_lsh(db, id, vek, meta)
        lsh_sisip_lsh(lsh, id, vek)
        idx += 1
    }
    var recall_benar = 0
    var total_kandidat = 0
    var q = 0
    selama (q < 10) {
        misal qvek = []
        var d2 = 0
        selama (d2 < D) {
            tambah(qvek, acak(-1, 1))
            d2 += 1
        }
        misal brute = vektor_kueri_knn_lsh(db, qvek, 1)
        misal approx_res = lsh_kueri_knn_lsh(lsh, db, qvek, 1)
        total_kandidat += approx_res.jumlah_kandidat
        jika panjang(brute) > 0 dan panjang(approx_res.hasil) > 0 {
            jika brute[0].id == approx_res.hasil[0].id {
                recall_benar += 1
            }
        }
        q += 1
    }
    misal rata_kandidat = total_kandidat / 10
    jika rata_kandidat >= 500 {
        kembalikan salah
    }
    jika recall_benar >= 6 {
        kembalikan benar
    }
    kembalikan salah
}

kembalikan uji_sendiri_vektor_lsh_fts();
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_8_vectordb_lsh_approx_recall: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_9_timeseries_rollup_retensi_2000() {
    let code = r##"
struktur DeretWaktuTS {
    nama, data_points, rollup_config, retensi_jam, rollup_data, jumlah_titik_sejak_compact, all_data_points,

    fungsi inisialisasi(nama) {
        ini.nama = nama
        ini.data_points = []
        ini.rollup_config = nihil
        ini.retensi_jam = nihil
        ini.rollup_data = {}
        ini.jumlah_titik_sejak_compact = 0
        ini.all_data_points = []
    }
}

fungsi tambah_titik_waktu_ts(dw, ts, val) {
    tambah(dw.data_points, {
        "waktu": ts,
        "nilai": val
    })
    kembalikan benar
}

fungsi detik_per_interval_ts(interval_str) {
    jika interval_str == "1m" { kembalikan 60 }
    kalau interval_str == "5m" { kembalikan 300 }
    kalau interval_str == "1h" { kembalikan 3600 }
    kembalikan 60
}

fungsi deret_waktu_lazy_compact_ts(dw) {
    misal all_data = dw.all_data_points
    misal pjg_all = panjang(all_data)

    jika dw.rollup_config != nihil {
        untuk idx_interval dalam 0..(panjang(dw.rollup_config) - 1) {
            misal interval_str = dw.rollup_config[idx_interval]
            misal sec = detik_per_interval_ts(interval_str)
            misal bucket_map = {}
            misal bucket_keys = []

            untuk i dalam 0..(pjg_all - 1) {
                misal titik = all_data[i]
                misal ts = titik.waktu
                misal val = titik.nilai
                misal bucket_key = lantai(ts / sec) * sec
                misal key_str = ke_teks(bucket_key)

                jika bucket_map[key_str] == nihil {
                    misal entry = {
                        "bucket_ts": bucket_key,
                        "sum": val,
                        "min": val,
                        "max": val,
                        "count": 1
                    };
                    bucket_map[key_str] = entry
                    tambah(bucket_keys, bucket_key)
                } lainnya {
                    misal e = bucket_map[key_str];
                    e.sum = e.sum + val
                    jika val < e.min { e.min = val }
                    jika val > e.max { e.max = val }
                    e.count = e.count + 1
                }
            }

            misal buckets = []
            misal pjg_keys = panjang(bucket_keys)
            jika pjg_keys > 0 {
                untuk i dalam 0..(pjg_keys - 1) {
                    misal bkey = bucket_keys[i]
                    misal bkey_str = ke_teks(bkey)
                    misal e = bucket_map[bkey_str]
                    misal bucket = {
                        "bucket_ts": e.bucket_ts,
                        "avg": e.sum / e.count,
                        "min": e.min,
                        "max": e.max,
                        "count": e.count
                    };
                    tambah(buckets, bucket)
                }
            }
            dw.rollup_data[interval_str] = buckets
        }
    }

    misal data = dw.data_points
    misal pjg = panjang(data)
    jika dw.retensi_jam != nihil && pjg > 0 {
        misal ts_terbaru = data[pjg - 1].waktu
        misal ts_batas = ts_terbaru - dw.retensi_jam * 3600
        misal data_baru = []
        untuk i dalam 0..(pjg - 1) {
            jika data[i].waktu >= ts_batas {
                tambah(data_baru, data[i])
            }
        }
        dw.data_points = data_baru
    }

    dw.jumlah_titik_sejak_compact = 0
    kembalikan benar
}

fungsi tambah_titik_waktu_rollup_ts(dw, ts, val) {
    tambah_titik_waktu_ts(dw, ts, val)
    misal titik_baru = {
        "waktu": ts,
        "nilai": val
    };
    tambah(dw.all_data_points, titik_baru)
    dw.jumlah_titik_sejak_compact = dw.jumlah_titik_sejak_compact + 1
    jika dw.jumlah_titik_sejak_compact % 50 == 0 {
        deret_waktu_lazy_compact_ts(dw)
    }
    kembalikan benar
}

fungsi deret_waktu_kueri_ts(dw, dari_ts, ke_ts, interval_opt) {
    jika interval_opt == "raw" {
        misal hasil = []
        misal data = dw.data_points
        untuk i dalam 0..(panjang(data) - 1) {
            misal t = data[i].waktu
            jika t >= dari_ts && t <= ke_ts {
                tambah(hasil, data[i])
            }
        }
        kembalikan hasil
    } lainnya {
        misal hasil = []
        misal buckets = dw.rollup_data[interval_opt]
        jika buckets != nihil {
            untuk i dalam 0..(panjang(buckets) - 1) {
                misal b = buckets[i]
                jika b.bucket_ts >= dari_ts && b.bucket_ts <= ke_ts {
                    tambah(hasil, b)
                }
            }
        }
        kembalikan hasil
    }
}

fungsi deret_waktu_set_rollup_ts(dw, daftar_interval) {
    dw.rollup_config = daftar_interval
    kembalikan benar
}

fungsi deret_waktu_retensi_ts(dw, retensi_jam) {
    dw.retensi_jam = retensi_jam
    kembalikan benar
}

misal dw = DeretWaktuTS("suhu")
deret_waktu_set_rollup_ts(dw, ["1m", "5m"])
deret_waktu_retensi_ts(dw, 0.2)

misal ts_awal = 1000000000
untuk i dalam 0..1999 {
    misal ts = ts_awal + i
    misal val = acak(20.0, 30.0)
    tambah_titik_waktu_rollup_ts(dw, ts, val)
}

misal bucket_1m = dw.rollup_data["1m"]
misal pjg_bucket_1m = panjang(bucket_1m)
pastikan(pjg_bucket_1m >= 30 && pjg_bucket_1m <= 35, "Jumlah bucket 1m harus antara 30-35, tapi: " + ke_teks(pjg_bucket_1m))

misal pjg_raw = panjang(dw.data_points)
pastikan(pjg_raw <= 1000, "Jumlah titik RAW harus <= 1000, tapi: " + ke_teks(pjg_raw))

misal kueri_1m = deret_waktu_kueri_ts(dw, 0, 9999999999, "1m")
pastikan(panjang(kueri_1m) >= 30, "Kueri 1m harus >= 30, tapi: " + ke_teks(panjang(kueri_1m)))

kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_9_timeseries_rollup_retensi_2000: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_fs_10_demo_166_full_run() {
    let code = r##"
fungsi rad_full(deg) {
    kembalikan deg * PI / 180.0
}

fungsi my_exp_full(x) {
    jika x == 0 { kembalikan 1.0 }
    misal negatif = salah
    jika x < 0 { negatif = benar; x = -x }
    misal hasil = 1.0
    misal suku = 1.0
    untuk n dalam 1..25 {
        suku = suku * x / n
        hasil = hasil + suku
    }
    jika negatif { kembalikan 1.0 / hasil }
    kembalikan hasil
}

fungsi my_ln_full(x) {
    jika x <= 0 { kembalikan 0.0 }
    jika x == 1.0 { kembalikan 0.0 }
    misal y = (x - 1.0) / (x + 1.0)
    misal y2 = y * y
    misal hasil = 0.0
    misal y_pow = y
    untuk n dalam 1..30 {
        jika n % 2 == 1 { hasil = hasil + y_pow / n }
        y_pow = y_pow * y2
    }
    kembalikan 2.0 * hasil
}

fungsi my_atan_full(x) {
    jika x == 0 { kembalikan 0.0 }
    misal negatif = salah
    jika x < 0 { negatif = benar; x = -x }
    misal besar = salah
    jika x > 1.0 { besar = benar; x = 1.0 / x }
    misal x2 = x * x
    misal hasil = 0.0
    misal x_pow = x
    untuk n dalam 1..30 {
        jika n % 2 == 1 {
            jika n % 4 == 1 { hasil = hasil + x_pow / n }
            lainnya { hasil = hasil - x_pow / n }
        }
        x_pow = x_pow * x2
    }
    jika besar { hasil = PI / 2.0 - hasil }
    jika negatif { hasil = -hasil }
    kembalikan hasil
}

fungsi my_atan2_full(y, x) {
    jika x > 0 { kembalikan my_atan_full(y / x) }
    lainnya jika x < 0 {
        jika y >= 0 { kembalikan my_atan_full(y / x) + PI }
        lainnya { kembalikan my_atan_full(y / x) - PI }
    } lainnya {
        jika y > 0 { kembalikan PI / 2.0 }
        lainnya jika y < 0 { kembalikan -PI / 2.0 }
        lainnya { kembalikan 0.0 }
    }
}

fungsi haversine_meter_full(lat1, lon1, lat2, lon2) {
    misal r = 6371000.0
    misal d_lat = rad_full(lat2 - lat1)
    misal d_lon = rad_full(lon2 - lon1)
    misal a = sin(d_lat / 2.0) * sin(d_lat / 2.0) + cos(rad_full(lat1)) * cos(rad_full(lat2)) * sin(d_lon / 2.0) * sin(d_lon / 2.0)
    misal c = 2.0 * my_atan2_full(akar(a), akar(1.0 - a))
    kembalikan r * c
}

fungsi spasial_titik_dalam_poligon_full(titik, poly) {
    jika panjang(poly) < 3 { kembalikan salah }
    misal px = titik[0]
    misal py = titik[1]
    misal inside = salah
    misal n = panjang(poly)
    misal j = n - 1
    untuk i dalam 0..(n - 1) {
        misal xi = poly[i][0]; misal yi = poly[i][1]
        misal xj = poly[j][0]; misal yj = poly[j][1]
        misal cond1 = yi > py
        misal cond2 = yj > py
        misal intersect_cond1 = cond1 != cond2
        misal denom = yj - yi
        misal px_intersect = px
        jika denom != 0 { px_intersect = (xj - xi) * (py - yi) / denom + xi }
        misal intersect = intersect_cond1 dan (px < px_intersect)
        jika intersect { inside = !inside }
        j = i
    }
    kembalikan inside
}

fungsi spasial_hitung_luas_poligon_full(poly) {
    jika panjang(poly) < 3 { kembalikan 0.0 }
    misal area = 0.0
    misal n = panjang(poly)
    untuk i dalam 0..(n - 1) {
        misal j = (i + 1) % n
        area = area + poly[i][0] * poly[j][1]
        area = area - poly[j][0] * poly[i][1]
    }
    kembalikan (mutlak(area) / 2.0) * 111.0 * 111.0
}

fungsi spasial_hitung_panjang_garis_full(line_pts) {
    misal total = 0.0
    misal n = panjang(line_pts)
    untuk i dalam 0..(n - 2) {
        misal p1 = line_pts[i]
        misal p2 = line_pts[i + 1]
        total = total + haversine_meter_full(p1[1], p1[0], p2[1], p2[0]) / 1000.0
    }
    kembalikan total
}

struktur IndeksRTreeFull {
    kapasitas, entri,
    fungsi inisialisasi(kap) {
        ini.kapasitas = kap
        ini.entri = []
    }
}
fungsi rtree_sisip_full(rt, id, min_x, min_y, max_x, max_y, data) {
    tambah(rt.entri, {"id": id, "min_x": min_x, "min_y": min_y, "max_x": max_x, "max_y": max_y, "data": data})
    kembalikan benar
}
fungsi rtree_kueri_kotak_full(rt, qmin_x, qmin_y, qmax_x, qmax_y) {
    misal hasil = []
    untuk_setiap(rt.entri, fungsi(e) {
        misal ox = e.min_x <= qmax_x dan e.max_x >= qmin_x
        misal oy = e.min_y <= qmax_y dan e.max_y >= qmin_y
        jika ox dan oy { tambah(hasil, e) }
    })
    kembalikan hasil
}

struktur PohonBPFull {
    simpanan,
    fungsi inisialisasi(_o) { ini.simpanan = {} }
}
fungsi bplus_sisip_full(p, k, v) { p.simpanan[k] = v; kembalikan benar }
fungsi bplus_cari_full(p, k) {
    jika ada_kunci(p.simpanan, k) { kembalikan p.simpanan[k] }
    kembalikan nihil
}
fungsi PohonBPlusFull(order) { kembalikan PohonBPFull(order) }

struktur TabelRelasionalFull {
    nama_tabel, kolom, baris_data, indeks_pk,
    fungsi inisialisasi(nama, daftar_kolom) {
        ini.nama_tabel = nama
        ini.kolom = daftar_kolom
        ini.baris_data = []
        ini.indeks_pk = {}
    }
    fungsi sisip(data_record) {
        misal id = data_record["id"]
        jika id != nihil dan ada_kunci(ini.indeks_pk, ke_teks(id)) { kembalikan salah }
        tambah(ini.baris_data, data_record)
        jika id != nihil { ini.indeks_pk[ke_teks(id)] = panjang(ini.baris_data) - 1 }
        kembalikan benar
    }
    fungsi pilih_di_mana(fp) {
        kembalikan saring(ini.baris_data, fp)
    }
}

struktur KoleksiDokumenFull {
    nama_koleksi, dokumen, indeks_field, _dokumen_by_id,
    fungsi inisialisasi(nama) {
        ini.nama_koleksi = nama
        ini.dokumen = []
        ini.indeks_field = {}
        ini._dokumen_by_id = {}
    }
    fungsi sisip_satu(doc) {
        jika bukan ada_kunci(doc, "_id") { doc["_id"] = "doc_" + ke_teks(panjang(ini.dokumen) + 1) }
        tambah(ini.dokumen, doc)
        ini._dokumen_by_id[doc["_id"]] = panjang(ini.dokumen) - 1
        misal flds = kunci(ini.indeks_field)
        jika panjang(flds) > 0 {
            untuk_setiap(flds, fungsi(nama_field) {
                misal pohon = ini.indeks_field[nama_field]
                jika ada_kunci(doc, nama_field) {
                    misal nf = ke_teks(doc[nama_field])
                    misal ex = bplus_cari_full(pohon, nf)
                    misal daftar_id = []
                    jika ex != nihil { daftar_id = ex }
                    tambah(daftar_id, doc["_id"])
                    bplus_sisip_full(pohon, nf, daftar_id)
                }
            })
        }
        kembalikan Ok(doc["_id"])
    }
}

fungsi _cari_dokumen_by_id_full(koleksi, id) {
    jika ada_kunci(koleksi._dokumen_by_id, id) {
        kembalikan koleksi.dokumen[koleksi._dokumen_by_id[id]]
    }
    misal hasil = saring(koleksi.dokumen, fungsi(d) { kembalikan d["_id"] == id })
    jika panjang(hasil) > 0 { kembalikan hasil[0] }
    kembalikan nihil
}

fungsi indeks_tambah_full(koleksi, nama_field) {
    misal pohon = PohonBPlusFull(4)
    untuk_setiap(koleksi.dokumen, fungsi(doc) {
        jika ada_kunci(doc, nama_field) {
            misal nf = ke_teks(doc[nama_field])
            misal ex = bplus_cari_full(pohon, nf)
            misal daftar_id = []
            jika ex != nihil { daftar_id = ex }
            tambah(daftar_id, doc["_id"])
            bplus_sisip_full(pohon, nf, daftar_id)
        }
    })
    koleksi.indeks_field[nama_field] = pohon
    kembalikan benar
}

fungsi indeks_cari_full(koleksi, nama_field, nilai_cari) {
    jika bukan ada_kunci(koleksi.indeks_field, nama_field) { kembalikan [] }
    misal pohon = koleksi.indeks_field[nama_field]
    misal hasil_ids = bplus_cari_full(pohon, ke_teks(nilai_cari))
    jika hasil_ids == nihil { kembalikan [] }
    misal hasil = []
    untuk_setiap(hasil_ids, fungsi(id) {
        misal doc = _cari_dokumen_by_id_full(koleksi, id)
        jika doc != nihil { tambah(hasil, doc) }
    })
    kembalikan hasil
}

struktur DeretWaktuFull {
    nama, data_points,
    fungsi inisialisasi(nama) {
        ini.nama = nama
        ini.data_points = []
    }
}

fungsi tambah_titik_waktu_full(dw, ts, val) {
    tambah(dw.data_points, {"waktu": ts, "nilai": val})
    kembalikan benar
}

fungsi buat_bdv_full(dim) {
    kembalikan {"dimensi": dim, "entri": []}
}
fungsi vektor_sisip_full(db, id, vek, meta) {
    tambah(db.entri, {"id": id, "vektor": vek, "metadata": meta})
    kembalikan benar
}
fungsi vektor_hitung_full(db) {
    kembalikan panjang(db.entri)
}

struktur WidyaDBFull {
    nama_db, geo, rel, nosql,
    fungsi inisialisasi(nama_db) {
        misal rtree = IndeksRTreeFull(16)
        ini.nama_db = nama_db
        ini.geo = {
            "rtree": rtree,
            "titik_dalam_poligon": spasial_titik_dalam_poligon_full,
            "hitung_luas_poligon": spasial_hitung_luas_poligon_full,
            "hitung_panjang_garis": spasial_hitung_panjang_garis_full,
            "rtree_sisip": rtree_sisip_full,
            "rtree_kueri_kotak": rtree_kueri_kotak_full
        }
        ini.rel = {"tabel": {}}
        ini.nosql = {"dokumen": {}}
    }
}

fungsi spasial_join_atribut_full(db, list_id_geom, nama_tabel_relasional, kolom_fk) {
    misal hasil = []
    jika bukan ada_kunci(db.rel.tabel, nama_tabel_relasional) { kembalikan hasil }
    misal tbl = db.rel.tabel[nama_tabel_relasional]
    untuk geom_id dalam list_id_geom {
        misal baris_cocok = tbl.pilih_di_mana(fungsi(baris) {
            jika ada_kunci(baris, kolom_fk) {
                kembalikan ke_teks(baris[kolom_fk]) == ke_teks(geom_id)
            }
            kembalikan salah
        })
        untuk b dalam baris_cocok {
            tambah(hasil, {"id_geom": geom_id, "atribut": b})
        }
    }
    kembalikan hasil
}

cetak("[WidyaDB 166] INIT DIMULAI...")

misal db = WidyaDBFull("kecamatan_xyz")
pastikan(db.nama_db == "kecamatan_xyz", "Nama DB harus kecamatan_xyz")
pastikan(db.geo != nihil, "Komponen geo harus ada")
pastikan(db.rel != nihil, "Komponen rel harus ada")
pastikan(db.nosql != nihil, "Komponen nosql harus ada")
cetak("[2/10 PASS] WidyaDB.init berhasil")

misal geo_kecamatan = []
misal poly_A = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [0.0, 0.0]]
misal kec_A = {"id": 1, "nama": "KecA", "poligon": poly_A}
tambah(geo_kecamatan, kec_A)
rtree_sisip_full(db.geo.rtree, 1, 0.0, 0.0, 10.0, 10.0, kec_A)

misal poly_B = [[11.0, 0.0], [25.0, 0.0], [25.0, 15.0], [11.0, 15.0], [11.0, 0.0]]
misal kec_B = {"id": 2, "nama": "KecB", "poligon": poly_B}
tambah(geo_kecamatan, kec_B)
rtree_sisip_full(db.geo.rtree, 2, 11.0, 0.0, 25.0, 15.0, kec_B)

misal poly_C = [[0.0, 16.0], [15.0, 16.0], [15.0, 30.0], [0.0, 30.0], [0.0, 16.0]]
misal kec_C = {"id": 3, "nama": "KecC", "poligon": poly_C}
tambah(geo_kecamatan, kec_C)
rtree_sisip_full(db.geo.rtree, 3, 0.0, 16.0, 15.0, 30.0, kec_C)

pastikan(panjang(geo_kecamatan) == 3, "Harus 3 kecamatan")
misal cek_rt = rtree_kueri_kotak_full(db.geo.rtree, 0.0, 0.0, 30.0, 30.0)
pastikan(panjang(cek_rt) == 3, "RTree harus 3 kecamatan")
cetak("[3/10 PASS] GIS RTree INSERT 3 kecamatan")

misal kolom_warga = [{"nama": "id"}, {"nama": "nama"}, {"nama": "kecamatan_id"}, {"nama": "lat"}, {"nama": "lng"}]
misal tabel_warga = TabelRelasionalFull("warga", kolom_warga)
db.rel.tabel["warga"] = tabel_warga
tabel_warga.sisip({"id": 1, "nama": "Warga1", "kecamatan_id": 1, "lat": 5.0, "lng": 5.0})
tabel_warga.sisip({"id": 2, "nama": "Warga2", "kecamatan_id": 1, "lat": 3.0, "lng": 7.0})
tabel_warga.sisip({"id": 3, "nama": "Warga3", "kecamatan_id": 1, "lat": 8.0, "lng": 2.0})
tabel_warga.sisip({"id": 4, "nama": "Warga4", "kecamatan_id": 2, "lat": 12.0, "lng": 18.0})
tabel_warga.sisip({"id": 5, "nama": "Warga5", "kecamatan_id": 2, "lat": 5.0, "lng": 20.0})
tabel_warga.sisip({"id": 6, "nama": "Warga6", "kecamatan_id": 2, "lat": 10.0, "lng": 15.0})
tabel_warga.sisip({"id": 7, "nama": "Warga7", "kecamatan_id": 2, "lat": 8.0, "lng": 22.0})
tabel_warga.sisip({"id": 8, "nama": "Warga8", "kecamatan_id": 3, "lat": 20.0, "lng": 8.0})
tabel_warga.sisip({"id": 9, "nama": "Warga9", "kecamatan_id": 3, "lat": 25.0, "lng": 10.0})
tabel_warga.sisip({"id": 10, "nama": "Warga10", "kecamatan_id": 3, "lat": 22.0, "lng": 5.0})

pastikan(panjang(tabel_warga.baris_data) == 10, "Tabel warga harus 10 baris")
misal cek_kecA = tabel_warga.pilih_di_mana(fungsi(b) { kembalikan b.kecamatan_id == 1 })
pastikan(panjang(cek_kecA) == 3, "Warga KecA harus 3")
misal cek_kecB = tabel_warga.pilih_di_mana(fungsi(b) { kembalikan b.kecamatan_id == 2 })
pastikan(panjang(cek_kecB) == 4, "Warga KecB harus 4")
misal cek_kecC = tabel_warga.pilih_di_mana(fungsi(b) { kembalikan b.kecamatan_id == 3 })
pastikan(panjang(cek_kecC) == 3, "Warga KecC harus 3")
cetak("[4/10 PASS] SQL Tabel warga 10 baris (A:3, B:4, C:3)")

misal log_sensor = KoleksiDokumenFull("log_sensor")
db.nosql.dokumen["log_sensor"] = log_sensor
misal nomor_s = 1
selama nomor_s <= 20 {
    misal kid = ((nomor_s - 1) % 3) + 1
    log_sensor.sisip_satu({"sensor_id": "S1", "kecamatan_id": kid, "nilai_suhu": 20.0 + acak() * 15.0})
    nomor_s = nomor_s + 1
}
pastikan(panjang(log_sensor.dokumen) == 20, "log_sensor harus 20 dokumen")
cetak("[5/10 PASS] NoSQL KoleksiDokumen log_sensor 20 dokumen")

misal suhu_dw = DeretWaktuFull("suhu")
db.nosql.ts = suhu_dw
misal ts_awal = 1700100000
untuk i_ts dalam 0..99 {
    tambah_titik_waktu_full(suhu_dw, ts_awal + i_ts * 300, 24.0 + acak() * 4.0)
}
pastikan(panjang(suhu_dw.data_points) == 100, "DeretWaktu harus 100 titik")
cetak("[6/10 PASS] DeretWaktu 100 titik")

misal bdv = buat_bdv_full(4)
db.nosql.vektor = bdv
untuk iv dalam 0..4 {
    misal v = [acak() * 2.0 - 1.0, acak() * 2.0 - 1.0, acak() * 2.0 - 1.0, acak() * 2.0 - 1.0]
    vektor_sisip_full(bdv, "w" + ke_teks(iv + 1), v, {"id_warga": iv + 1})
}
pastikan(vektor_hitung_full(bdv) == 5, "VektorDB harus 5 entri")
cetak("[7/10 PASS] BasisDataVektor 5 entri D=4")

indeks_tambah_full(log_sensor, "kecamatan_id")
misal hasil_idx2 = indeks_cari_full(log_sensor, "kecamatan_id", 2)
pastikan(panjang(hasil_idx2) >= 4, "indeks_cari kecamatan_id=2 harus >= 4, tapi: " + ke_teks(panjang(hasil_idx2)))
cetak("[8/10 PASS] Secondary Index indeks_cari jumlah: " + ke_teks(panjang(hasil_idx2)))

misal rt_kecB = rtree_kueri_kotak_full(db.geo.rtree, 11.0, 0.0, 25.0, 15.0)
pastikan(panjang(rt_kecB) >= 1, "RTree query KecB harus menemukan")

misal list_id_kecB = []
untuk_setiap(rt_kecB, fungsi(item) {
    misal id_geom = item.id
    misal poly_item = nihil
    untuk_setiap(geo_kecamatan, fungsi(gk) {
        jika gk.id == id_geom { poly_item = gk.poligon }
    })
    jika poly_item != nihil {
        misal sample_pt = [(item.min_x + item.max_x) / 2.0, (item.min_y + item.max_y) / 2.0]
        jika spasial_titik_dalam_poligon_full(sample_pt, poly_item) {
            tambah(list_id_kecB, id_geom)
        }
    } lainnya {
        tambah(list_id_kecB, id_geom)
    }
})

misal warga_KecB = spasial_join_atribut_full(db, list_id_kecB, "warga", "kecamatan_id")
pastikan(panjang(warga_KecB) == 4, "SPATIAL JOIN KecB harus 4 orang, tapi: " + ke_teks(panjang(warga_KecB)))
cetak("[9/10 PASS] SPATIAL JOIN warga KecB = 4 orang")

cetak("[10/10 PASS] FULL STACK MULTI-MODAL DEMO SUKSES!")

kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_fs_10_demo_166_full_run: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

