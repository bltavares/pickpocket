use std::env;

const HEADER: &'static str = r#"
<!DOCTYPE NETSCAPE-Bookmark-file-1>

<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">

<TITLE>Bookmarks</TITLE>

<H1>Bookmarks</H1>

<DL><p>


"#;

const FOOTER: &'static str = r#"
</DL><p>
"#;

fn main() {
    let csv_file_name = env::args()
        .nth(1)
        .expect("Expected an csv file as argument");

    let csv_reader = csv::Reader::from_path(csv_file_name);

    let mut entries = Vec::with_capacity(50000);

    for item in csv_reader.expect("couldnt read csv").records() {
        let item = item.expect("coudltn read line");
        let item = item.iter().collect::<Vec<_>>();
        let [title, url, time_added, tags, status] = &item[..] else {
            panic!("Malformed CSV");
        };

        let toread = if *status == "unread" { "1" } else { "0" };
        let tags = tags.replace('|', ",");
        let tags = if *status == "archive" {
            format!("linkding:bookmarks.archived,{}", tags)
        } else {
            tags
        };

        let entry = format!(
            r#"
        <DT><A HREF="{url}" ADD_DATE="{time_added}" PRIVATE="1" TOREAD="{toread}" TAGS="{tags}">{title}</A>
"#,
        );

        entries.push(entry);
    }

    println!("{}{}{}", HEADER, entries.join("\n"), FOOTER);
}
