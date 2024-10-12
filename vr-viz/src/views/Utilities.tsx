import {SendJsonMessage} from "react-use-websocket/dist/lib/types";

function UtilitiesDisplay({setter}: { setter: SendJsonMessage }) {
    return (
        <div className="padding-around flex flex-col gap-2">
            <button onClick={() => setter({
                AskPin: {
                    length: 3
                }
            })}>Begin Pinentry
            </button>
        </div>
    )
}

export default UtilitiesDisplay;