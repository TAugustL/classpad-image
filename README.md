<h1>Introduction</h1>
<div>
  The fx-CP400 calculator can not use PNG or JPG files, it can, however, use the CASIO C2P format instead.
  While there is offical software by CASIO available for converting images to c2p, these programs require a special password that is supposed to be given by your teacher.
  This project is based on <a href="https://github.com/the6p4c/Kalkimg">this repository by the6p4c</a>. It only requires that you have Rust installed and a fx-CP400 calculator (with an USB cable).
</div>

<h1>How to use</h1>
<div>
  <ol>
    <li>
      install <a href="https://www.rust-lang.org/tools/install">Rust</a> (if not already done)
    </li>
    <li>
      enter a folder of your choice, open your terminal and enter "git clone https://github.com/TAugustL/c2p-converter.git"
    </li>
    <li>
      enter the created folder, in your terminal enter "cargo run [path/to/image.png] [target/path/image.c2p]"
    </li>
  </ol>
  
  <p>Now you have your image as an c2p file. In the next steps you will need your calculator.</p>
  
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
      drag your c2p file in there
    </li>
    <li>
      securely remove your calculator from your PC
    </li>
  </ol>

  <p>And done! Now you can open the image on your calculator!</p>
  
</div>

<h1>How does it work?</h1>
<div>
  For the file to be valid it needs to have 4 things:
  <ul>
    <li>
      a valid size (max. 310 x 401), however if you use my program it will scale images that are too large down
    </li>
    <li>
      the c2p header (some bytes change depending on your file size!)
    </li>
    <li>
      a zlib encryption (on default compression mode)
    </li>
    <li>
      a c2p footer (same in every c2p file)
    </li>
  </ul>
  </ul>
  <a href="https://cdn.ti-planet.org/downloads2/1406770160/Classpad.c2pimagefileformat.pdf">Here are the necessary bites for your file</a> (mind the bytes that change in the header)
</div>

<br>
<hr>
<p>If you have any tips for improvement or issues please open an issue!</p>
