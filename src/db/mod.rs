use std::fs;
use crate::model::Movie;

static Movies_DB: &str = "data/movies.json";

fn movies () -> Result<Vec<Movie> , serde_json::Error> {
    let data = fs::read_to_string(Movies_DB).expect("Error reading");
    let movies: Result<Vec<Movie> , serde_json::Error> = serde_json::from_str(&data);
    movies
} 

fn write_movies (movies: Vec<Movie>)  {
    let data = serde_json::to_string(&movies).expect("Error reading");
    fs::write(Movies_DB , data).expect("Failed to write the data");
} 

pub fn read_movies() -> Option<Vec<Movie>> {
    match movies() {
        Ok(movies) => Some(movies),
        Err(_) => None

    }
}

    pub fn read_movie(title: String) -> Option<Movie> {
        match movies(){
            Ok(movies) => {
                let index = movies.iter().position(|m| m.title == title);
                match index  {

                Some(x) =>  Some(movies[x].clone()),
                None => None, 

        }
    },
    Err(_) => None
    }
}

pub fn insert_movie(movie: Movie) -> Option<Movie> {
    match movies(){
        Ok(mut movies) => {
            movies.push(movie.clone());
            write_movies(movies);
            Some(movie)
        },
        Err(_) => None
    }
}

pub fn delete_movie(title: String) -> bool {
    match movies() {
        Ok(mut movies) => {
            let index = movies.iter().position(|m|m.title == title);
        
            match index {
                Some(x) => {
                    movies.remove(x);
                    write_movies(movies);
                    true
                },
                None => false,              
  
            }

        },

        Err(_) => false
    }
}