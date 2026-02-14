use clap::Parser;
use ed25519_dalek::{Signer, SigningKey, Signature};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use rand::rngs::OsRng;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Generate {
        #[arg(long)]
        output_private: PathBuf,
        #[arg(long)]
        output_public: PathBuf,
    },
    Sign {
        #[arg(long)]
        private_key: PathBuf,
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output_metadata: PathBuf,
        #[arg(long)]
        version: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebContainerMetadata {
    pub version: u32,
    pub signature: Signature,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { output_private, output_public } => {
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);
            
            // Save raw bytes
            File::create(output_private)?.write_all(&signing_key.to_bytes())?;
            File::create(output_public)?.write_all(signing_key.verifying_key().as_bytes())?;
            
            println!("Generated new keypair.");
        }
        Commands::Sign { private_key, input, output_metadata, version } => {
            // Read private key
            let mut sk_bytes = [0u8; 32];
            File::open(private_key)?.read_exact(&mut sk_bytes)?;
            let signing_key = SigningKey::from_bytes(&sk_bytes);

            // Read input content
            let mut content = Vec::new();
            File::open(input)?.read_to_end(&mut content)?;

            // Construct message: [version (4 bytes BE)] + [content]
            let mut message = version.to_be_bytes().to_vec();
            message.extend_from_slice(&content);

            // Sign
            let signature = signing_key.sign(&message);

            // Create metadata
            let metadata = WebContainerMetadata {
                version,
                signature,
            };

            // Serialize metadata to CBOR
            let mut metadata_file = File::create(output_metadata)?;
            ciborium::ser::into_writer(&metadata, &mut metadata_file)?;
            
            println!("Signed content. Metadata written.");
        }
    }

    Ok(())
}
