import sys
import re
import http.client
from markdownify import markdownify as md

# Check if the correct number of arguments is provided
if len(sys.argv) < 3 or len(sys.argv) > 4:
    print('Usage: python download_desc.py <year> <day> [session]')
    sys.exit(1)  # Exit with an error code

# Parse arguments
year = int(sys.argv[1])
day = int(sys.argv[2])

# Check if year is within the valid range
if not (2015 <= year <= 3000):
    print('Error: Year must be an integer between 2015 and 3000.')
    sys.exit(1)

# Check if day is within the valid range
if not (1 <= day <= 25):
    print('Error: Day must be an integer between 1 and 25.')
    sys.exit(1)

# Optional: Check and parse session if provided
session = None
if len(sys.argv) == 4:
    provided_session = sys.argv[3]
    if not all(c in '0123456789abcdefABCDEF' for c in provided_session) or len(provided_session) != 128:
        print('Error: session must be a hexadecimal string of 128 characters.')
        sys.exit(1)
    session = provided_session

url = f'https://adventofcode.com/{year}/day/{day}'

headers = {}
if session:
    headers['Cookie'] = f'session={session}'

connection = http.client.HTTPSConnection("adventofcode.com")
connection.request("GET", url, headers=headers)

response = connection.getresponse()
html = response.read().decode('utf-8')

connection.close()

pos = 0
content = ''

openTag = '<article class="day-desc">'
closeTag = '</article>'

while True:
    start_index = html.find(openTag, pos)
    if start_index == -1:
        break
    end_index = html.find(closeTag, start_index + len(openTag))
    content += html[start_index + len(openTag):end_index]
    pos = end_index + len(closeTag)

# ensure code blocks arent interpreted as rust doc-tests
md_text = md(content, code_language="ignore")

# The website uses <code><em> for bold code blocks the resulting markdown `*...*` wouldnt be parsed correctly so flip it inside out
md_text = re.sub(r"`\*(.*?)\*`", "**`\\g<1>`**", md_text, 0, re.MULTILINE)

print(md_text)
