# RusTureng

[![GitHub](https://img.shields.io/github/license/tunakasif/rustureng)](https://github.com/tunakasif/rustureng/blob/main/LICENSE)

*Blazingly fast* and unnecessarily over-engineered CLI tool for using [tureng.com](https://tureng.com/en/turkish-english), written in *✨Rust✨*. [Tureng](https://tureng.com/en/turkish-english) is a commonly known and highly regarded translation site in Türkiye, famous for its idiomatic translations. [Tureng Multilingual Dictionary](https://tureng.com/en/turkish-english) offers an extensive dictionary to search terms in English, French, German, Spanish, and Turkish. However, in its current form, the implementation targets `<tur-eng>` and `<eng-tur>` translations. Other languages can be used by altering the base URL, although some unwanted side effects or deficient output may occur.

## ⚙️ Usage

To search for a term, provide the desired `<term>` in the following format. If the term exists, the tables of translations are outputted, where an interactive selection pane is prompted when the term is missing but closely relates to other entries and indicates the term is not found if the given term does not exist.

```sh
cargo run --release -- <term>
```

## ⚖️ Legality

- [Tureng's terms of use](https://tureng.com/en/termsofuse) indicates that all content provided by Tureng on the website, as well as the website itself, are protected by the laws on copyright, trademark, and other applicable intellectual property and proprietary rights. Also, they indicate that users may not use automated scripts or programs to screen scrape or otherwise extract data from pages on the Tureng website.

- The aim of this program is to easily search for translations without leaving your cozy terminal screen. Therefore, it is not a tool for the systematic scraping of the website. Using this tool for that purpose would violate the terms of use of the website. In its provided form, the program sends a `GET` request to the website with the given term and reorganizes the received HTML file. In other words, it is no different than visiting the site on a browser with disabled JavaScript or using an ad blocker.
