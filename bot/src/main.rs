use octocrab::{models::repos::Object, params::repos::Reference, Octocrab};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");

    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let refs = octocrab
        .repos("ekuinox", "goshuin")
        .get_ref(&Reference::Branch("deploy-1".into()))
        .await?;

    let sha = match refs.object {
        Object::Commit { sha, .. } => sha,
        Object::Tag { sha, .. } => sha,
        _ => return Ok(()),
    };

    let content = octocrab
        .repos("ekuinox", "goshuin")
        .get_content()
        .path("hello_1")
        .r#ref(sha)
        .send()
        .await?;

    let item = content.items.first().expect("msg");
    let mut content = item
        .content
        .to_owned()
        .and_then(|c| {
            // 改行コードが 60 文字区切りで入っているので消していく
            let c = c
                .chars()
                .into_iter()
                .filter(|c| *c != '\n')
                .collect::<String>();
            let decoded = base64::decode(c);
            decoded.ok().and_then(|s| String::from_utf8(s).ok())
        })
        .unwrap_or_default();
    println!("{}", content);

    content += "hello world\n";

    let repo = octocrab.repos("ekuinox", "goshuin");
    let a = repo
        .update_file("hello_1", "append hello", content, item.sha.to_owned())
        .branch("deploy-1")
        .send()
        .await;
    println!("{:?}", a);

    Ok(())
}

#[test]
fn a() {
    println!("{:?}", base64::encode("hello world").bytes());

    let bytes = base64::decode("aGVsbG8gd29ybGQK").unwrap();
    println!("{:?}", bytes);
}
