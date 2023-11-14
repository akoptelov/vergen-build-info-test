fn main() {
    println!("Commit SHA:       {}", env!("VERGEN_GIT_SHA"));
    println!("Commit branch:    {}", env!("VERGEN_GIT_BRANCH"));
    println!("Commit timestamp: {}", env!("VERGEN_GIT_COMMIT_TIMESTAMP"));
}
