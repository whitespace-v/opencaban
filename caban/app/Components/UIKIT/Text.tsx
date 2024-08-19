import classes from '../../styles/Components/UIKIT/Text.module.scss'
interface IProps {
    children: string
    color?: "cyan" | "white" | "gradient"
    fz?: "fzhuge" | "fzbig" | "fzmed" | "fzsmol"
    fw?: "fwbig" | "fwmed" | "fwsmol"
    font?: "mono"
}

const Text = (props: IProps) => {
    const cls = [
        classes['Text'],
        classes[props.color || ''],
        classes[props.font || ''],
        classes[props.fz || ''],
        classes[props.fw || '']
    ]
    return <p className={cls.join(' ')}> {props.children.trim()} </p >
}
export default Text