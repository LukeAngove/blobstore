use blobgraph::blobstore::BlobStore;
use flate2::read::ZlibDecoder;
use gitstore::git_files::GitStore;
use gix_date::{time::Sign, Time};
use gix_object::{tree::EntryMode, Kind, Object, ObjectRef};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

fn sj_reader(a: BufReader<File>) -> Result<Object, Box<dyn Error>> {
    let mut decompressor = ZlibDecoder::new(a);
    let mut data = Vec::<u8>::new();
    let _ = decompressor.read_to_end(&mut data);
    let end_kind = data
        .iter()
        .position(|x| *x == b' ')
        .ok_or("No spaces present.")?;
    let kind_str = &data[0..end_kind];
    let kind = Kind::from_bytes(kind_str)?;
    let start = data.iter().position(|x| *x == b'\0').ok_or("No zero.")? + 1;
    let res = ObjectRef::from_bytes(kind, &data[start..])?;
    let x = Ok(res.into());
    x
}

fn hourmin_to_seconds(hourmin: i32) -> i32 {
    (hourmin / 100) * (60 * 60)
}

fn make_store() -> GitStore {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../.git/objects");
    GitStore::new(d.display().to_string())
}

#[test]
fn test_read_commit() {
    let main_store = make_store();

    let id = "08912e3ade4f1bb17c5c54e982ff0863a76b9124".to_string();

    let val = main_store
        .get(&id, sj_reader)
        .expect("Failed to get result");

    let tree = "bcca3ba7e6e9154c0d2eae7b56c2148853f8ec29";
    let parent = "79ab2eccf433f3e6522f9e5e3f0922494d6bf254";
    let author_name = "Luke Angove";
    let author_email = "luke.angove@gmail.com";
    let author_time = Time {
        seconds: 1696936661,
        sign: Sign::Plus,
        offset: hourmin_to_seconds(1100),
    };
    let committer_name = "Luke Angove";
    let committer_email = "luke.angove@gmail.com";
    let committer_time = Time {
        seconds: 1696936661,
        sign: Sign::Plus,
        offset: hourmin_to_seconds(1100),
    };
    let message = "Add hash store that returns an id and basic tests\n";

    assert_eq!(Kind::Commit, val.kind());
    if let Object::Commit(commit) = val {
        assert_eq!(tree.to_string(), commit.tree.to_string());
        assert_eq!(1, commit.parents.len());
        assert_eq!(parent, commit.parents[0].to_string());
        assert_eq!(author_name, commit.author.name.to_string());
        assert_eq!(author_email, commit.author.email.to_string());
        assert_eq!(author_time, commit.author.time);
        assert_eq!(committer_name, commit.committer.name.to_string());
        assert_eq!(committer_email, commit.committer.email.to_string());
        assert_eq!(committer_time, commit.committer.time);
        assert_eq!(message, commit.message.to_string());
    } else {
        panic!("Incorrect type for value.");
    }
}

#[test]
fn test_read_tree() {
    let main_store = make_store();

    let id = "0f20c99188e8b701d3edb87c832b4feb0c50b183".to_string();

    let val = main_store
        .get(&id, sj_reader)
        .expect("Failed to get result");

    let num_entries = 6;
    let names = vec![
        ".gitignore",
        "Cargo.lock",
        "Cargo.toml",
        "README.md",
        "src",
        "tests",
    ];
    let shas = vec![
        "ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba",
        "c82a649469e9242c567392bdaa0224f6a97224de",
        "4726e18bbc1eb2f87a4d91b3fb493d2fb70c6421",
        "3a97862f0f20f324a6149dd1e9e1e6a7f2eea0d2",
        "4662705aeb3c982f2c8ecf8a697d695f77983967",
        "890ed7f1357b0dfaa137dab8883ec1436f97def8",
    ];
    let modes = vec![
        EntryMode::Blob,
        EntryMode::Blob,
        EntryMode::Blob,
        EntryMode::Blob,
        EntryMode::Tree,
        EntryMode::Tree,
    ];

    assert_eq!(Kind::Tree, val.kind());
    if let Object::Tree(tree) = val {
        assert_eq!(num_entries, tree.entries.len());
        assert_eq!(
            names,
            tree.entries
                .iter()
                .map(|a| { a.filename.to_string() })
                .collect::<Vec<String>>()
        );
        assert_eq!(
            shas,
            tree.entries
                .iter()
                .map(|a| { a.oid.to_string() })
                .collect::<Vec<String>>()
        );
        assert_eq!(
            modes,
            tree.entries
                .iter()
                .map(|a| { a.mode })
                .collect::<Vec<EntryMode>>()
        );
    } else {
        panic!("Incorrect type for value.");
    }
}

#[test]
fn test_read_blob() {
    let main_store = make_store();

    let id = "3812e82491517fff0ce0b9c87461b303edba434f".to_string();

    let val = main_store
        .get(&id, sj_reader)
        .expect("Failed to get result");

    let data = r#"[workspace]

resolver = "2"
members = [
  "blobgraph",
]

"#
    .to_string();
    assert_eq!(Kind::Blob, val.kind());
    if let Object::Blob(blob) = val {
        assert_eq!(
            data,
            String::from_utf8(blob.data).expect("Expected string data.")
        );
    } else {
        panic!("Incorrect type for value.");
    }
}
