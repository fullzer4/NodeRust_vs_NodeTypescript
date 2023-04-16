use neon::prelude::*;

fn minha_funcao(mut cx: FunctionContext) -> JsResult<JsString> {
    let array_handle = cx.argument::<JsArray>(0)?;
    let len = array_handle.len();
    let mut result = String::new();
    
    for i in 0..len {
        let string_handle = array_handle.get(&mut cx, i)?;
        let string = string_handle.downcast::<JsString>().or_throw(&mut cx)?;
        let value = string.value();
        result.push_str(&value);
    }

    Ok(cx.string(result))
}

register_module!(mut cx, {
    cx.export_function("minhaFuncao", minha_funcao)
});