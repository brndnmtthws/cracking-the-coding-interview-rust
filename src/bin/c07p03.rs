use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
struct Song {
    title: String,
    artist: String,
    album: String,
    year: String,
}

#[derive(Debug, Clone)]
struct Jukebox {
    catalogue: Vec<Song>,
    queue: Vec<Song>,
}

impl Jukebox {
    fn new() -> Jukebox {
        Jukebox {
            catalogue: vec![
                Song {
                    title: "I'm creepy".to_string(),
                    artist: "Taylor Swift".to_string(),
                    album: "Tay Tay Sings Songs".to_string(),
                    year: "2020".to_string(),
                },
                Song {
                    title: "Baby".to_string(),
                    artist: "Justin Bieber".to_string(),
                    album: "Baby Songs".to_string(),
                    year: "2009".to_string(),
                },
                Song {
                    title: "Hit Me Baby (One More Time)".to_string(),
                    artist: "Britney Spears".to_string(),
                    album: "Spear Songs".to_string(),
                    year: "2000".to_string(),
                },
            ],
            queue: vec![],
        }
    }

    fn next_song(&mut self) -> Option<Song> {
        if self.queue.is_empty() {
            // pick random song
            let mut rng = thread_rng();
            if let Some(song) = self.catalogue.choose(&mut rng) {
                Some(song.clone())
            } else {
                None
            }
        } else {
            Some(self.queue.remove(0))
        }
    }

    fn enqueue_song(&mut self, song: Song) {
        self.queue.push(song);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jukebox() {
        let mut jukebox = Jukebox::new();

        // Get a shuffled song
        let song = jukebox.next_song().unwrap();
        assert_eq!(song.year.starts_with("20"), true);

        // Add a song to the queue, and fetch it
        jukebox.enqueue_song(jukebox.catalogue[1].clone());
        let next_song = jukebox.next_song().unwrap();
        assert_eq!(next_song.title, "Baby");
    }
}

fn main() {
    let mut jukebox = Jukebox::new();

    // Get a shuffled song
    let _song = jukebox.next_song().unwrap();

    // Add a song to the queue, and fetch it
    jukebox.enqueue_song(jukebox.catalogue[1].clone());
    let _next_song = jukebox.next_song().unwrap();
}
