import {toast} from "sonner";

export function terror(message: string) {
    toast.error(message, {
        richColors: true,
    })
    console.error(message)
}

export function tinfo(message: string) {
    toast.info(message, {
        richColors: true,
    })
    console.info(message)
}

export function twarning(message: string) {
    toast.warning(message, {
        richColors: true,
    })
    console.warn(message)
}

export function tmessage(message: string) {
    toast.message(message, {
        richColors: true,
    })
    console.log(message)
}

export function tsuccess(message: string) {
    toast.success(message, {
        richColors: true,
    })
    console.log(message)
}
