from phonology_engine import PhonologyEngine
from pprint import pprint
pe = PhonologyEngine()
res = pe.process(u'gera')
pprint(res.__next__()[0][0]["stress_options"])
# for word_details, phrase, normalized_phrase, letter_map in res:
#     pprint(normalized_phrase)
#     pprint(letter_map)
#     pprint(letter_map)
#     for word_detail in word_details:
#         pprint(word_detail)
