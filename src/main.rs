use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;
struct Site {
    acctno: String,
    domain: String,
    name: String,
    nation: String,
    found: u64,
}
fn main() {
    let vargs: Vec<String> = std::env::args().collect();

    if vargs.len() < 3 {
        println!("usage: 文件名 记录数");
        process::exit(1);
        // panic!("输入参数错误")
    }
    println!("文件名:{}", vargs[1]);
    println!("记录数:{}", vargs[2]);

    let mut runoob = Site {
        acctno: String::from("10000"),
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };

    let fname: String = vargs[1].trim().parse().unwrap();
    let nums = vargs[2].trim().parse().unwrap();

    // let mut file = File::create(vargs[1].as_str()).unwrap();
    // 创建文件，打开读写创建权限
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fname)
        .unwrap();
    // 取当前日期时间
    let fmt = "%Y-%m-%d %H:%M:%S%.3f";
    let now = Local::now();
    let str_date = now.format(fmt).to_string();
    println!("{}", str_date);
    // 按记录数写指定格式数据
    for i in 0..nums {
        // runoob.acctno = format!("{0}{1:>0width$}", "62312233", i, width = 10);
        runoob.acctno = format!("{}{:>010}", "62312233", i);
        runoob.found += i;
        // 随机取日期信息
        let now = Local::now();
        let str_date = now.format(fmt).to_string();
        // 组织行数据,写入文件
        let lines = format!(
            "{}|{}|{}|{}|{}|{}|{}",
            i, runoob.acctno, runoob.domain, runoob.name, runoob.nation, runoob.found, str_date
        );
        writeln!(file, "{}", lines).unwrap();
    }
    // 写文件完成后的时间
    let now = Local::now();
    let str_date = now.format(fmt).to_string();
    println!("{}", str_date);
}
