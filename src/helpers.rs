pub fn current_time_string() -> String {
    time::OffsetDateTime::now_local()
        .format("%F %r")
        .to_string()
}

pub fn sanitize(s: &str) -> String {
    // This is used in a closure later.
    // To avoid the period as first character, we pretend that there had been
    // a period alread.
    let mut last_c = '.';

    // Proceed line by line.
    s.lines()
        .map(|l| {
            let mut s = l
                .chars()
                // Replace tab with space.
                .map(|c| if c.is_whitespace() { ' ' } else { c })
                // Delete control characters.
                .filter(|c| !c.is_control())
                // Replace `:\\/|?~,;=` with underscore.
                //
                // Exclude NTFS critical characters:       <>:"\\/|?*
                // https://msdn.microsoft.com/en-us/library/windows/desktop/aa365247%28v=vs.85%29.aspx
                // Exclude restricted in fat32:            +,;=[]
                // https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words
                // These are considered unsafe in URLs:    <>#%{}|\^~[]`
                // https://perishablepress.com/stop-using-unsafe-characters-in-urls/
                .map(|c| {
                    if c == ':'
                        || c == '\\'
                        || c == '/'
                        || c == '|'
                        || c == '?'
                        || c == '~'
                        || c == ','
                        || c == ';'
                        || c == '='
                    {
                        '_'
                    } else {
                        c
                    }
                })
                // Replace `<>:"#%{}^[]+\`` with space.
                //
                // Exclude NTFS critical characters:       <>:"\\/|?*
                // https://msdn.microsoft.com/en-us/library/windows/desktop/aa365247%28v=vs.85%29.aspx
                // Exclude restricted in fat32:            +,;=[]
                // https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words
                // These are considered unsafe in URLs:    <>#%{}|\^~[]`
                // https://perishablepress.com/stop-using-unsafe-characters-in-urls/
                .map(|c| {
                    if c == '<'
                        || c == '>'
                        || c == '"'
                        || c == '*'
                        || c == '#'
                        || c == '%'
                        || c == '{'
                        || c == '}'
                        || c == '^'
                        || c == '['
                        || c == ']'
                        || c == '+'
                        || c == '`'
                    {
                        ' '
                    } else {
                        c
                    }
                })
                // Filter space after space.
                // Filter period after period, space, underscore or beginning of the string.
                // Filter underscore after period, space or underscore.
                .filter(|&c| {
                    let discard = (c == ' ' && last_c == ' ')
                        || ((c == '_' || c == '.')
                            && (last_c == '.' || last_c == '_' || last_c == ' '));
                    if !discard {
                        last_c = c;
                    };
                    !discard
                })
                .collect::<String>()
                // Trim whitespace and `_-` at the beginning and the end of the line.
                .trim_matches(|c: char| c.is_whitespace() || c == '_' || c == '-')
                .to_string();
            // Filter newline and insert line speparator `-`.
            s.push('-');
            s
        })
        .collect::<String>()
        // Trim whitespace and `_-` at the beginning and the end of the whole string.
        .trim_matches(|c: char| c.is_whitespace() || c == '_' || c == '-')
        .to_string()
}
