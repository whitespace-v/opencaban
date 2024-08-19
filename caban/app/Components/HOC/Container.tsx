import classes from '../../styles/Components/HOC/Container.module.scss'
const Container = ({ children }: { children: React.ReactNode }) => {
    return (
        <div className={classes["Container"]}>
            {children}
        </div>
    )
}

export default Container