# distribute_pts_in_circle
Evenly distribute points in a circle

This code is a Rust port (with additional features) of the Python code from here:
https://stackoverflow.com/questions/28567166/uniformly-distribute-x-points-inside-a-circle
This code builds a command-line application that allows for the generation of PDF or SVG output images.

![Alt text](./sample.svg)

# Command-line options
`cargo run --help`

    Distribute points in a circle
    
    Usage: distribute-pts-in-circle [OPTIONS] <N> <COMMAND>
    
    Commands:
      svg   
      pdf   
      help  Print this message or the help of the given subcommand(s)
    
    Arguments:
      <N>  number of points

    Options:
      -a, --alpha <ALPHA>  boundary evenness [default: 0]
      -h, --help           Print help
      -V, --version        Print version

`cargo run help pdf`

    Usage: distribute-pts-in-circle <N> pdf [OPTIONS] <OUTPUT>
    
    Arguments:
      <OUTPUT>  PDF file path
    
    Options:
      -d, --dotsize <DOTSIZE>  dot size (in mm) [default: 0.5]
      -r, --radius <RADIUS>    circle radius (in mm) [default: 75]
      -h, --help               Print help

`cargo run help svg`

    Usage: distribute-pts-in-circle <N> svg [OPTIONS] <OUTPUT>

    Arguments:
      <OUTPUT>  SVG file path

    Options:
      -d, --dotsize <DOTSIZE>  dot size [default: 5]
      -h, --help               Print help
