#!/usr/bin/env python

"""Update the list of personal statement drafts in index.html."""

import re
from glob import glob


def main() -> None:
    """Update the list of personal statement drafts in index.html."""
    raw_pdfs: list[str] = []

    for pdf in glob('ps/dyson-draft-*.pdf'):
        raw_pdfs.append(pdf)

    pdfs: list[tuple[str, int]] = [
        (x, int(match.group(1)))
        for x in raw_pdfs
        if (match := re.match(r'ps/dyson-draft-([\d\.]+)\.pdf', x)) is not None
    ]

    pdfs.sort(key=lambda t: t[1])

    list_content = ''

    for name, number in pdfs:
        list_content += f'\t\t<li><a href="{name}" download>Draft {number}</a></li>\n'

    with open('index.html', 'r', encoding='utf-8') as f:
        new_index_html = re.sub(
            r'(?<=\t<ul id="ps-draft-list">\n)(\t\t<li><a href="ps/dyson-draft-[\d\.]+\.pdf" '
            r'download>Draft [\d\.]+</a></li>\n)*(?=\t</ul>)',
            list_content,
            f.read()
        )

    with open('index.html', 'w', encoding='utf-8') as f:
        f.write(new_index_html)


if __name__ == '__main__':
    main()
