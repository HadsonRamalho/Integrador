import { forwardRef, InputHTMLAttributes } from "react";

//inputprops recebe todas as propriedades de um elemento input do html
//
type InputProps = InputHTMLAttributes<HTMLInputElement>;

export const Input = forwardRef <HTMLInputElement, InputProps> (({name = '' ,type ='text' ,...props}, ref) =>{
    return(
        
        <input type={type} name={name} ref ={ref} {...props} />
    );
}
);


export default Input;