import Link from "next/link";
import classes from "../../styles/Components/UIKIT/Button.module.scss";

interface IButton {
    children: string;
    type: "box" | "text" | "round";
    color: "white" | "gray"
    onClick?: (e: React.MouseEvent<HTMLDivElement>) => void;
    to?: string
    icon?: React.ReactNode
}

const Button = (props: IButton) => {
    const cls = [
        classes['Button'],
        classes[props.type],
        classes[props.color],
    ]

    return (
        <>
            {props.to ?
                <Link className={cls.join(' ')} href={props.to}>{props.children}</Link> :
                <div className={cls.join(' ')} onClick={props.onClick}>
                    {props.children.trim()} {props.icon}
                </div>
            }
        </>
    );
};

export default Button;