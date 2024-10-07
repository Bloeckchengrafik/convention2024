import {ReactNode} from "react";

export function ButtonList({children}: { children: ReactNode }) {
    return <div className="flex gap-2 items-center justify-center">
        {children}
    </div>
}

export function Button({children, onClick}: { children: ReactNode, onClick: () => void }) {
    return <button onClick={onClick} className="text-sm p-2">{children}</button>
}