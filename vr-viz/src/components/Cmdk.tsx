import "react-cmdk/dist/cmdk.css";
import {DockviewApi} from "dockview";
import {useState} from "react";
import CommandPalette, {
    filterItems,
    getItemIndex,
    IconName,
    JsonStructureItem,
    useHandleOpenCommandPalette
} from "react-cmdk";
import {
    copyToClipboard,
    openGyroTab,
    openInferenceSettingsTab, openOptimizerInfoTab, openServoSettingsTab,
    openVrDistanceConfigurationTab, openWheelReadingTab,
    restoreDefaultLayout
} from "../dockviewapi.ts";

function Cmdk(props: { dockview: DockviewApi | undefined }) {
    const api = props.dockview;

    const [page] = useState<"root" | "projects">("root");
    const [open, setOpen] = useState<boolean>(false);
    const [search, setSearch] = useState("");

    useHandleOpenCommandPalette(setOpen);

    if (api === undefined) {
        return <div>Dockview not ready</div>
    }


    function view(name: string, icon: IconName, func: () => void): JsonStructureItem {
        return {
            id: name,
            children: name,
            icon,
            onClick: func,
        }
    }

    const filteredItems = filterItems(
        [
            {
                heading: "Views",
                id: "views",
                items: [
                    view("Gyro", "Square3Stack3DIcon", openGyroTab.bind(null, api)),
                    view("VR Distance Configuration", "ViewfinderCircleIcon", openVrDistanceConfigurationTab.bind(null, api)),
                    view("Inference Settings", "ServerIcon", openInferenceSettingsTab.bind(null, api)),
                    view("Wheel Reading", "EllipsisHorizontalCircleIcon", openWheelReadingTab.bind(null, api)),
                    view("Servo Config", "AdjustmentsHorizontalIcon", openServoSettingsTab.bind(null, api)),
                    view("Optimizer Info", "PresentationChartLineIcon", openOptimizerInfoTab.bind(null, api)),
                ],
            },
            {
                heading: "Commands",
                id: "commands",
                items: [
                    view("Copy to clipboard", "ClipboardIcon", copyToClipboard.bind(null, api)),
                    view("Restore default layout", "PaintBrushIcon", restoreDefaultLayout.bind(null, api)),
                ]
            }
        ],
        search
    );

    return (
        <CommandPalette
            onChangeSearch={setSearch}
            onChangeOpen={setOpen}
            search={search}
            isOpen={open}
            page={page}
        >
            <CommandPalette.Page id="root">
                {filteredItems.length ? (
                    filteredItems.map((list) => (
                        <CommandPalette.List key={list.id} heading={list.heading}>
                            {list.items.map(({id, ...rest}) => (
                                <CommandPalette.ListItem
                                    key={id}
                                    index={getItemIndex(filteredItems, id)}
                                    {...rest}
                                />
                            ))}
                        </CommandPalette.List>
                    ))
                ) : (
                    <CommandPalette.FreeSearchAction/>
                )}
            </CommandPalette.Page>
        </CommandPalette>
    );
}

export default Cmdk;