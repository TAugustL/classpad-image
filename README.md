<h1>Introduction</h1>
<div>
  The fx-CP400 calculator has the ability to display images within cerain programs. It can not use PNG or JPG files, it can, however, use the CASIO C2P format instead.
  There is also a second format, the C2B format, which is a animation file consisting of multiple images used in the 'Picture Plot' application.
  While there is offical software by CASIO available for converting images to C2P and C2B, these programs require a special password that is supposed to be given by your teacher.
  The C2P converter is based on <a href="https://github.com/the6p4c/Kalkimg">this repository by the6p4c</a>, the C2B converter was done entirely by myself. It only requires that you have Rust installed and a fx-CP400 calculator (with an USB cable).
</div>

<h1>How to use</h1>
<div>
  <ol>
    <li>
      install <a href="https://www.rust-lang.org/tools/install">Rust</a> (if not already done)
    </li>
    <li>
      enter a folder of your choice, open your terminal and enter: 
      
      git clone https://github.com/TAugustL/classpad-image.git
      
  </li>
    <li>
      <p>Into C2P: enter the created folder, in your terminal enter:</p>
      
      cargo run --example clt c2p [path/to/image.png] [target/path/image.c2p]"
      
  </li>
  <li>
      <p>Into C2B: enter the created folder, in your terminal enter:</p>
      
      cargo run --example clt c2b [path/to/image1.png] [path/to/image2.png] [path/to/image3.png] [target/path/image.c2b]"
      
  </li>
  <li>
      <p>From C2P: enter the created folder, in your terminal enter:</p>
      
      cargo run --example clt imgp [target/path/image.c2p] [path/to/image.png]"
      
  </li>
  <li>
      <p>From C2B: enter the created folder, in your terminal enter:</p>
      
      cargo run --example clt imgb [target/path/image.c2b] [path/to/image.png]"
      
  </li>
  </ol>
  
  <p>Now you have your image as an C2P file. In the next steps you will need your calculator.</p>
  
  <ol>
    <li>
      plug your calculator with the USB cable to your PC
    </li>
    <li>
      in the window that should now open, press the first option (USB Flash)
    </li>
    <li>
      open the USB Drive that should now appear on your PC
    </li>
    <li>
      drag your C2P or C2B file in there
    </li>
    <li>
      securely remove your calculator from your PC
    </li>
  </ol>

  <p>And done! Now you can open the image on your calculator!</p>
  
</div>

<h1>How does it work?</h1>
<div>
  There are some things in which C2P and C2B files are similar:
  <ul>
    <li>
      a valid size (max. 310 x 401), however if you use my program it will scale images that are too large down
    </li>
    <li>
      a specific header (some bytes change depending on your file size!)
    </li>
    <li>
      a zlib encryption for the image data (on default compression mode)
    </li>
    <li>
      a specific footer (same in every C2P/C2B file)*
    </li>
  </ul>
  <a href="https://cdn.ti-planet.org/downloads2/1406770160/Classpad.c2pimagefileformat.pdf">Here are the necessary bites for your file</a> (mind the bytes that change in the header)
  
  The C2B file format is a bit more complicated than the C2P format:
  <ul>
    <li>
      the file starts again with a header with some bytes changing depending on your file size
    </li>
    <li>
      following the header, there are four bytes containing the size of the following compressed picture
    </li>
    <li>
      at the end of the compressed data, this repeats for each picture (header, 4 bytes, data, 4 bytes, data, ..., footer)
    </li>
    <li>
      the file ends with a specific footer
      *this is the same for every C2B file, however, there are some bytes that have an effect on the environment inside the 'Picture Plot' application (some bytes change the window dimensions, no effect on picture itself)
    </li>
  </ul>
</div>

<br>
<hr>
<p>If you have any tips for improvement or issues please open an issue!</p>
