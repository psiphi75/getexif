# get exif

Get exif details from a JPG image file.

## Usage

```bash
cargo run --release -- samples/sample.jpg
```

## Todo

- Tests: I would add at least a test against `samples/sample.jpg` and it's
  corresponding `.json` output. 
- Better error handling.  Something like [this](https://github.com/psiphi75/smart-buoy/blob/master/src/errors.rs) for a small project.
- Better logging, probably use a logging framework.
- Fix issue with "capture_time" date format and timezone.  Need more info about
  how this works.
- Fix the `unwrap()`s in `main.rs`.  This can be easily done by printing an
  error for the file.  But need to think about what to do with the process 
  error code and how to propagate it out of the thread.  An 
  [atomic counter](https://doc.rust-lang.org/std/sync/atomic/) could work.
- The `exif::Fields` return an array of primatives, I'm not sure these were
  handled correctly.
- Have enabled `rayon` for multithreading.  If I had more time I'd look for a 
  crate/solution that has less dependencies, or at lease review rayon's 
  dependencies.  The thread count is set to 4, but probably needs some smarter
  thread handling such that this application doesn't bring the system to a 
  grinding halt when the user reads 1000 `.jpeg` files.  This all requires a
  bit of instrumentation, testing, monitoring, and tuning.

## Time

- Reading and planning 30 minutes. (30)
- 12:45 to 13:25 - First commit. (40)
- 13:50 to 14:30 - Second block. (40)
- 14:50 to 16:40 - Complete, but needs tidy. (110)
- 11:25 to 12:05 - Done. (30)
