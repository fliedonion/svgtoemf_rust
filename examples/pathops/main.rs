use std::path::Path;
use std::path::PathBuf;
use std::ffi::{OsString, OsStr};


fn test_path_valiation() {
    println!("");
    println!("test_path_valiation()");

    // let testf = |path| {
    //     let p = Path::new(path);
    //     println!("> extension of '{}' is {:?}", path, p.extension().unwrap_or_else(|| OsStr::new("")));
    //
    //     let mut pb = PathBuf::from(path);
    //     pb.set_extension("emf");
    //     println!("> extension replaced path is '{}'", &pb.to_string_lossy());
    // };

    // let testf = |path| {
    //     // inpath / exppath 使うときに参照を渡している。
    //     let inpath = format!("{}.svg", path);
    //     let exppath = format!("{}.emf", path);
    //
    //     let p = Path::new(&inpath);
    //     println!("> extension of '{}' is {:?}", &inpath, p.extension().unwrap_or_else(|| OsStr::new("")));
    //
    //     let mut pb = PathBuf::from(&inpath);
    //     pb.set_extension("emf");
    //     println!("> extension replaced path is '{}'", &pb.to_string_lossy());
    //     assert_eq!(&exppath, &pb.to_string_lossy());
    // };


    let testf = |inpath: &String, exppath: &String, expext: &str | {
        let p = Path::new(inpath);
        let ext = p.extension().unwrap_or_else(|| OsStr::new(""));
        println!("> extension of '{}' is {:?}", inpath, ext);
        assert_eq!(expext, ext.to_string_lossy());

        let mut pb = PathBuf::from(inpath);
        pb.set_extension("emf");
        println!("> extension replaced path is '{}'", &pb.to_string_lossy());
        assert_eq!(exppath, &pb.to_string_lossy());
    };


    let path = "\\\\vmware-host\\Shared Folders\\test\\test.file\\test";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");

    let path = "\\\\vmware-host\\Shared Folders\\test\\test.file\\test";
    let inpath = format!("{}", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "");



    let path = "A:\\test\\test.file\\test.svg"; // drive does not exist. but expect path can be made.
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = ".\\test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "C:\\Users\\fliedonion\\files\\..\\.\\..\\test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "t.e.s.t.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "/Users/fliedonion/Documents/test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "/home/fliedonion/some/directory/test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "../test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "file:///c:/somewhere/test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");


    let path = "file:////vmware-host/Shared%20Folders/somewhere/test.svg";
    let inpath = format!("{}.svg", path);
    let exppath = format!("{}.emf", path);
    testf(&inpath, &exppath, "svg");

}

fn test_path_exists() {

    let testf = |path, expect| {
        let p = Path::new(path);

        let actual = p.exists();
        assert_eq!(expect, actual, "{} exists not match. left is expected.", path);
    };
    let path = "C:\\windows\\system32\\notepad.exe";
    testf(path, true);

    let path = "C:\\windows\\system32\\not_found_app.exe";
    testf(path, false);

    println!("");
    println!("test_path_exists passed.")
}


fn strip_and_print_ext() {
    let path = "c:\\temp\\test.file\\test.txt";
    let p = Path::new(path);
    println!("extension of 'c:\\temp\\test.file\\test.txt' is {:?}", p.extension().unwrap_or_else(|| OsStr::new("")));

    let mut pb = PathBuf::from(path);
    pb.set_extension("xls");
    println!("{}", &pb.to_string_lossy());


    let path = "test.txt";
    let p = Path::new(path);

    println!("extension of 'test.txt' is {:?}", p.extension().unwrap_or_else(|| OsStr::new("")));

    let path = "test";
    let p = Path::new(path);

    println!("extension of 'test' is {:?}", p.extension().unwrap_or_else(|| OsStr::new("")));

    let path = ".gitignore";
    let p = Path::new(path);

    println!("extension of '.gitignore' is {:?}", p.extension().unwrap_or_else(|| OsStr::new("")));
}

fn strip_extenison_literal() -> &'static OsStr {
    // この場合 引数がリテラルに確定している。
    // リテラルだからString Objectではない。
    // リテラルは std::str である（逆か）。
    // リテラルは　デフォルトでは 本質的に静的であり、それゆえプログラム実行中 有効（is valid）である。
    // 戻り値のライフサイクルもstaticとできる。

    // この関数の戻り値の型を &OsStr とはできない。
    // （実際のところはリテラルから発生していてstaticにできるけれど、定義から見れば）
    // p は関数内で借用した変数だからこの関数とライフタイムが同じで関数が終わり次第なくなるので。
    // ヒープに確保した PathBuf（これだとブランクにはできなささそうだから Option<PathBuf>とかかな）なり 
    // OsStringなりで返すしかない。

    let p = Path::new("test.cmd");  
    p.extension().unwrap_or_else(|| OsStr::new(""))
}

fn strip_extenison(infile: &str) -> &OsStr {
    // これは引数次第なので、戻り値のライフサイクルにstaticをつける必要はない。

    let p = Path::new(infile);
    p.extension().unwrap_or_else(|| OsStr::new(""))
}

fn strip_extenison_str() -> &'static OsStr {
    // 変数に代入しているからではなく、リテラルから発生していることが確定しているので、
    // staticライフサイクルをつけろと言われる。

    let infile:&str = "test.svg";
    let p = Path::new(infile);  
    p.extension().unwrap_or_else(|| OsStr::new(""))
}


fn strip_extenison_strobject() -> PathBuf {

    let infile = String::from("test.emf");
    Path::new(&infile).to_path_buf()
}

fn strip_extenison_strobject2() -> OsString {
    // よくわかってないが、これだとinfileが borrowされた状態ではじまった
    // （たぶん）OsStr を外にムーブできない。
    // なのでPathBufを返すとか、OsStringを返す。（これらはムーブできる版）

    let infile = String::from("test.emf");
    let p = Path::new(&infile);
    p.extension().unwrap_or_else(|| OsStr::new("")).to_os_string()
}


fn use_strip_extension() {

    let pb = strip_extenison_strobject();
    println!("{:?}", pb.extension().unwrap_or_else(|| OsStr::new("")));
    println!("{:?}", strip_extenison_strobject2());


    println!("{:?}", strip_extenison_str());
    println!("{:?}", strip_extenison_literal());
    println!("{:?}", strip_extenison("test.log"));
}



fn main() {
    strip_and_print_ext();    
    use_strip_extension();
    test_path_valiation();
    test_path_exists();
}