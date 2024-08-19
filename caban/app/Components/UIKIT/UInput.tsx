import classes from '../../styles/Components/UIKIT/UInput.module.scss'
import { useState, } from 'react'
interface IProps {
    placeholder: string
    value: string
    setValue: (i: string) => void;
}
const UInput = (props: IProps) => {
    const [value, setValue] = useState<string>('')
    return <input className={classes['UInput']} type="text"
        placeholder={props.placeholder}
        value={value}
        onChange={e => setValue(e.currentTarget.value)}
    />
}

export default UInput