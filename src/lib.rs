use std::{fmt::{Display, Debug}, process::abort, };

use tract_onnx::prelude::{
    tvec, Datum, Framework, InferenceFact, InferenceModelExt, Tensor
};

#[no_mangle]
pub extern "C" fn foo(start: *const u8, len: u32, token_size: u32) -> u32 {
    //FIXME do not run this, this just makes sure the right code is compiled in
    let bytes = unsafe { ::std::slice::from_raw_parts(start, len as usize) };
    _foo(bytes, token_size as usize).ok();
    1
}

fn _foo(mut bytes: &[u8], token_size: usize) -> Result<u32, Abort> {
    let input_fact = InferenceFact::dt_shape(i64::datum_type(), &[1, token_size]);

    let plan = tract_onnx::onnx()
        .model_for_read(&mut bytes)?
        .with_input_fact(0, input_fact.clone())? // token ids
        .with_input_fact(1, input_fact)? // attention mask
        .into_optimized()?
        .into_runnable()?;


    if plan.model().output_fact(0)?.shape.as_concrete()
        != Some(&[1, token_size, 768])
    {
        Err("foobar")?;
    }


    let t1 = Tensor::zero::<i64>(&[2, 2])?;
    let t2 = Tensor::zero::<i64>(&[2, 2])?;
    let outputs = plan.run(tvec![t1, t2])?;

    Ok(outputs[0].as_slice()?.iter().fold(0, |a,b|a.wrapping_add(*b)))
}

struct Abort {
    _priv: (),
}

impl<Error> From<Error> for Abort
where
    Error: Display+Debug,
{
    fn from(err: Error) -> Self {
        eprintln!("An error occurred aborting: {}\nDebug({:?})", err, err);
        abort()
    }
}

