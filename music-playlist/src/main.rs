struct Song {
    title: String,
    artist: String,
    duration: u32, // in seconds
    next: Option<Box<Song>>,
}

struct Playlist {
    name: String,
    head: Option<Box<Song>>,
}

impl Playlist {
    fn new(name: String) -> Self {
        Playlist { name, head: None }
    }

    fn add_song(&mut self, title: String, artist: String, duration: u32) {
        let new_song = Box::new(Song {
            title,
            artist,
            duration,
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(new_song);
            return;
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_song);
    }

    fn remove_song(&mut self, title: &str) {
        let mut current = &mut self.head;
        let mut prev: Option<Box<Song>> = None;

        while let Some(mut node) = current.take() {
            if node.title == title {
                if let Some(mut prev_node) = prev {
                    prev_node.next = node.next.take();
                } else {
                    self.head = node.next.take();
                }
                return;
            }

            // Move ownership of `node` into `prev`
            prev = Some(node);
            current = &mut prev.as_mut().unwrap().next;
        }

        println!("Song '{}' not found in playlist.", title);
    }

    fn play(&self) {
        println!("Playing playlist: {}", self.name);
        let mut current = &self.head;
        while let Some(node) = current {
            println!(
                " - {} by {} ({} seconds)",
                node.title, node.artist, node.duration
            );
            current = &node.next;
        }
    }

    // Optional methods
    fn insert_song_after() {
        todo!()
    }

    fn shuffle() {
        todo!()
    }
}

fn main() {
    let mut playlist = Playlist::new("My Favorite Songs".to_string());

    playlist.add_song(
        "Bohemian Rhapsody".to_string(),
        "Queen".to_string(),
        409_600,
    );
    playlist.add_song("Revolver".to_string(), "The Beatles".to_string(), 297_920);
    playlist.add_song(
        "Smells Like Teen Spirit".to_string(),
        "Nirvana".to_string(),
        300_736,
    );

    playlist.play();

    playlist.remove_song("Bohemian Rhapsody");
    playlist.play();

    // Optional methods
    // playlist.insert_song_after();
    // playlist.shuffle();
}
