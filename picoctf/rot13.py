cipher = {
   'a' : 'n',
   'b' : 'o',
   'c' : 'p',
   'd' : 'q',
   'e' : 'r',
   'f' : 's',
   'g' : 't',
   'h' : 'u',
   'i' : 'v',
   'j' : 'w',
   'k' : 'x',
   'l' : 'y',
   'm' : 'z',
   'n' : 'a',
   'o' : 'b',
   'p' : 'c',
   'q' : 'd',
   'r' : 'e',
   's' : 'f',
   't' : 'g',
   'u' : 'h',
   'v' : 'i',
   'w' : 'j',
   'x' : 'k',
   'y' : 'l',
'z' : 'm',
   'A' : 'N',
   'B' : 'O',
   'C' : 'P',
   'D' : 'Q',
   'E' : 'R',
   'F' : 'S',
   'G' : 'T',
   'H' : 'U',
   'I' : 'V',
   'J' : 'W',
   'K' : 'X',
   'L' : 'Y',
   'M' : 'Z',
   'N' : 'A',
   'O' : 'B',
   'P' : 'C',
   'Q' : 'D',
   'R' : 'E',
   'S' : 'F',
   'T' : 'G',
   'U' : 'H',
   'V' : 'I',
   'W' : 'J',
   'X' : 'K',
   'Y' : 'L',
    'Z' : 'M'
}

def solve(text):
    output = ""
    for char in text:
        if char in cipher.keys():
            value = cipher[char]
            output += value
        else:
            output += char
    return output

input = "cvpbPGS{arkg_gvzr_V'yy_gel_2_ebhaqf_bs_ebg13_GYpXOHqX}"
solution = solve(input)
print(solution)
