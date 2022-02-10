<div id="top"></div>


<h3 align="center">Equistitch</h3>

  <p align="center">
    Equistitch is utility for breaking 360 degree equirectangular panoramas into perspective projected cubemaps, tiles and making the transformation back.
    <br />
    <a href="https://github.com/emblica/equistitch"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/emblica/equistitch/issues">Report Bug</a>
    ·
    <a href="https://github.com/emblica/equistitch/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This is small utility tool made to process 360 degree images. It can be used as component of the data pipeline in ML recognition from 360 degree images.

There can be other use-cases as well if you need to process perspective fixed images and then stitch them back.

You could also generate cubemaps for 3D engines using this tool.

<p align="right">(<a href="#top">back to top</a>)</p>



### Built With

* [Rust](https://www.rust-lang.org/)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Equistitch is very simple program. Only single file `src/main.rs`

To use either build from sources or download binary.

### Prerequisites

You just need Rust toolkit installed.
Check out also binary releases

### Installation from sources

1. Install rust toolkit (https://www.rust-lang.org/tools/install)
2. Clone the repo
   ```sh
   git clone https://github.com/emblica/equistitch.git
   ```
3. Build
   ```sh
   cargo build --release
   ```
4. Equistitch will be built at `target/release/equistitch`

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Show help
```
equistitch -h
```

Equirectangular image to cubemap faces:
```
equistitch split --input example.png --cubemap-faces-output cube/
```

Equirectangular image to tiles:
```
equistitch split --input example.png --tiles-output tiles/
```

Cubemap faces to equirectangular image:
```
equistitch stitch --input-dir cube/ --output exa_stitch_from_cubemap.png
```

Tiles to equirectangular image:
```
equistitch stitch --input-dir tiles/ -t --output exa_stitch_from_tiles.png
```


<p align="right">(<a href="#top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

Unix principle is strong in this one. Equistitch does single task and it does it well, there is not planned roadmap currently.

Bug fixes and performance updates are possible.

See the [open issues](https://github.com/emblica/equistitch/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Emblica - [@emblicacom](https://twitter.com/emblicacom) - hello@emblica.com

Project Link: [https://github.com/emblica/equistitch](https://github.com/emblica/equistitch)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/emblica/equistitch.svg?style=for-the-badge
[contributors-url]: https://github.com/emblica/equistitch/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/emblica/equistitch.svg?style=for-the-badge
[forks-url]: https://github.com/emblica/equistitch/network/members
[stars-shield]: https://img.shields.io/github/stars/emblica/equistitch.svg?style=for-the-badge
[stars-url]: https://github.com/emblica/equistitch/stargazers
[issues-shield]: https://img.shields.io/github/issues/emblica/equistitch.svg?style=for-the-badge
[issues-url]: https://github.com/emblica/equistitch/issues
[license-shield]: https://img.shields.io/github/license/emblica/equistitch.svg?style=for-the-badge
[license-url]: https://github.com/emblica/equistitch/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/company/emblica