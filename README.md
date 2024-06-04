# University of Puget Sound Sustainability Services course ranker thing
## Coded up by Matilda Lerner~

Yay!! A real usable project that I'm getting paid for!!!

This program takes a course catalog and a list of keywords with affiliated scores, 
then scores courses using the keywords in the course-description.

### Usage
To run this program, you'll need two things:
- A file full of scored keywords
- The link to the University of Puget Sound's bulletin page

The link's been hardcoded, as I assume it won't be changing anytime soon.
The keywords file, on the other hand, is a little tricky, so I made a separate section for it below.
> cargo run \<link to keywords file> (department codes)

It is also worth noting that multiple department codes can be used subsequently, separated by spaces.
Only the codes will work, and they must be in all caps (i.e. AFAM, ENGL).

### Keywords file
One of the arguments when running the program is the path to a "keywords file." 
This is essentially the file containing all the keywords and their affiliated scores.
The format is as follows:
- Star (*) followed by the score
- All keywords of the given score, listed on one line, separated by semicolons

Here's an example keywords file:
> *1
> planet; nature;
>
> *5
> humans; environment; global;

This info is stored in a plain text file (.txt), and the path is then specified in the arguments.

### Packages used in this project
- [ureq](https://crates.io/crates/ureq): Used for pulling HTML data
