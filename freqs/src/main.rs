use freqs::config;
use tokio::{fs::File, io::{self, AsyncReadExt}, sync::mpsc::{self, Receiver, Sender}, task::JoinHandle};
use std::collections::HashMap;
use std::io::Write;

const FILES_BUFFER_SIZE: usize = 8;
const WORDS_BUFFER_SIZE: usize = 5000;

async fn read_files(filenames: Vec<String>, sender: Sender<String>) -> io::Result<()> {
    for filename in filenames {
        let mut file = File::open(filename).await?;
        let mut text = String::new();
        file.read_to_string(&mut text).await?;

        if sender.send(text).await.is_err() {
            break;
        }
    }

    Ok(())
}

async fn split_into_words(sender: Sender<String>, mut texts_recv: Receiver<String>) {
    while let Some(string) = texts_recv.recv().await {
        for word in string.split_ascii_whitespace().collect::<Vec<_>>() {
            if sender.send(word.to_owned()).await.is_err() {
                break;
            }
        }
    }
}

async fn split_into_letters(mut words_recv: Receiver<String>) -> HashMap<char, usize>{
    let mut freqs: HashMap<char, usize> = HashMap::new();

    while let Some(word) = words_recv.recv().await {
        for letter in word.chars().collect::<Vec<char>>() {
            if letter.is_alphanumeric() {
                if let Some(val) = freqs.get(&letter){
                    freqs.insert(letter, val + 1); 
                } else {
                    freqs.insert(letter, 1);
                }
            }
        }
    }

    freqs
}

/// Start a thread that reads files into memory.
fn start_file_reader_thread(filenames: Vec<String>) -> Receiver<String> {
    let (sender, receiver) = mpsc::channel(FILES_BUFFER_SIZE);
    tokio::spawn(read_files(filenames, sender));

    receiver
}
/// Start a thread that receives text and splits the text into words.
fn start_text_splitter_thread(texts_recv: Receiver<String>) -> Receiver<String> {
    let (sender, receiver) = mpsc::channel(WORDS_BUFFER_SIZE);
    tokio::spawn(split_into_words(sender, texts_recv) );

    receiver
}
/// Start a thread that receives words and splits the words into letters.
fn start_word_splitter_thread(words_recv: Receiver<String>) -> JoinHandle<HashMap<char, usize>> {
    tokio::spawn(split_into_letters(words_recv))
}

fn run_pipeline(filenames: Vec<String>) -> JoinHandle<HashMap<char, usize>>{
    let texts_recv = start_file_reader_thread(filenames);
    let words_recv = start_text_splitter_thread(texts_recv);
    start_word_splitter_thread(words_recv)
}

fn write_to_file(freqs: HashMap<char, usize>) -> Result<(), tokio::io::Error> {
    let mut file = std::fs::File::create("freqs.csv")?;

    for (k,v) in freqs {
        write!(file, "{},{}\n", k, v)?;
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error>{
  let filenames = config::get_args();  

  let freqs = run_pipeline(filenames).await?;
  write_to_file(freqs)?;

  Ok(())
}
