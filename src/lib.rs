#[macro_use] extern crate cpython;
extern crate curl;
extern crate serde;
extern crate serde_json;
pub mod utils;

use cpython::{Python, PyDict, PyList, PyResult, ToPyObject, PyObject};
use curl::http;
use std::collections::BTreeMap;
use serde_json::Value;

fn retpy(py: Python) -> PyResult<String> {
    let locals = PyDict::new(py);
    locals.set_item(py, "os", py.import("os")?)?;
    let user: String = py.eval("os.getenv('HOME')", None, Some(&locals))?.extract(py)?;
    Ok(user.to_string()) 
}

fn returl(py: Python) -> PyResult<PyObject> {
    let url = "https://api.chucknorris.io/jokes/random";
    let resp = http::handle()
     .get(url)
     .exec()
     .unwrap_or_else(|e| {
         panic!("Falha ao abrir URL {}; Erro: {}", url, e);
    });
    let body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
        panic!("Falha ao interpretar resultado da URL {}; Erro: {}", url, e);
    });

    let test: Value = serde_json::from_str(&body).unwrap();

    let ret = utils::fromjson(py, test);

    Ok(ret)
}

fn retmapa(py: Python) -> PyResult<PyDict> {
    let dicionario = PyDict::new(py);
    dicionario.set_item(py, "chave1", "valor1")?;
    dicionario.set_item(py, "chave2", 123)?;
    Ok(dicionario)
}

fn reclista(py: Python, lista_python: Vec<i64>) -> PyResult<PyList> {
    let lista_soma = lista_python.iter().map(|&x| x + x).collect::<Vec<i64>>();
    let lista_retorno = lista_soma.to_py_object(py);
    Ok(lista_retorno)
}

fn btree_mapa(py: Python) -> PyResult<PyDict> {
    let mut dicionario_btree = BTreeMap::<String, i32>::new();
    dicionario_btree.insert("key".to_string(), 1);
    let py_map = dicionario_btree.to_py_object(py);
    Ok(py_map)
}

fn rec_btree_mapa(py: Python, mapa: PyDict) -> PyResult<PyDict> {
    let mut dicionario_btree = BTreeMap::<String, i32>::new();
    let res = mapa.items(py);
    for f in res.iter() {
        dicionario_btree.insert(f.0.to_string(), f.1.to_string().parse::<i32>().unwrap()+10);
    }
    let ret_map = dicionario_btree.to_py_object(py);
    Ok(ret_map)
}

py_module_initializer!(mvrpl, initmvrpl, PyInit_mvrpl, |py, m| {
    try!(m.add(py, "__doc__", "Este módulo foi escrito em Rust."));
    try!(m.add(py, "retpy", py_fn!(py, retpy())));
    try!(m.add(py, "retmapa", py_fn!(py, retmapa())));
    try!(m.add(py, "returl", py_fn!(py, returl())));
    try!(m.add(py, "reclista", py_fn!(py, reclista(rand_int:Vec<i64>))));
    try!(m.add(py, "mapa", py_fn!(py, btree_mapa())));
    try!(m.add(py, "recmapa", py_fn!(py, rec_btree_mapa(rand_int:PyDict))));
    Ok(())
});
