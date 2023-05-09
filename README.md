# numpad-annoying-word-sorter
a rust script that sorts a list of words by how time consuming they would be to type on a numerical keypad

how the words are rated:
the most time-consuming part of typing on a numerical keyboard for most is waiting for the cooldown to input the next letter when it occupies the same key as the previous.
<br>ex. typing 'B' directly after 'A'.

this algorithm sorts the words by the amount of consecutive same-key letter pairs. Of the words with the same amount, they are sorted by word length.

*A same-key letter pair is a pair of letters that occupy the same key on a numerical pad. i.e 'A' and 'B'

example output.txt:
```
pseudomonocotyledonous
nonaccommodatingness
nontrigonometrically
...
za
zo
a
```
