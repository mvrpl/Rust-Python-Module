import mvrpl
import json

print("------------- Retorna DocString do modulo -------------")
print(mvrpl.__doc__)
print

print("------------- Retorna resultado Python -------------")
print(mvrpl.retpy())
print

print("------------- Retorna Mapa Python -------------")
for k,v in mvrpl.retmapa().items():
    print("Chave: %s" % k)
    print("Valor: %s" % v)
    print

print("------------- Retorna Lista Python -------------")
print("__".join([str(n) for n in mvrpl.reclista([1,2,3])]))
print

print("------------- Retorna resposta de URL -------------")
print(mvrpl.returl())
print

print("------------- Retorna Mapa Rust no Python -------------")
print(mvrpl.mapa())
print

print("------------- Retorna parse dict Python -------------")
print(mvrpl.recmapa({'key':15, 'ds': 5}))
print