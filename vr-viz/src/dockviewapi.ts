import {AddPanelOptions, DockviewApi, Orientation} from "dockview";

const gyroSettings = {
    "id": "gyro",
    "component": "gyro",
    "title": "Gyroscope",
}

const vrdcSettings = {
    "id": "vrdc",
    "component": "vrdc",
    "title": "VR Distance Configuration",
}

function openOrCreatePanel(api: DockviewApi, panelDefinition: AddPanelOptions) {
    const panelId = panelDefinition.id;
    const panel = api.getPanel(panelId);

    if (panel === undefined) {
        api.addPanel(panelDefinition);
    } else {
        panel.api.setActive()
    }
}

export function openGyroTab(api: DockviewApi) {
    openOrCreatePanel(api, gyroSettings);
}

export function openVrDistanceConfigurationTab(api: DockviewApi) {
    openOrCreatePanel(api, vrdcSettings);
}

export function copyToClipboard(api: DockviewApi) {
    const json = api.toJSON();
    navigator.clipboard.writeText(JSON.stringify(json, null, 2)).then(() => console.log("Copied to clipboard"));
}

export function restoreDefaultLayout(api: DockviewApi) {
    api.fromJSON({
        "grid": {
            "root": {
                "type": "branch",
                "data": [
                    {
                        "type": "branch",
                        "data": [
                            {
                                "type": "leaf",
                                "data": {
                                    "views": [
                                        "gyro"
                                    ],
                                    "activeView": "gyro",
                                    "id": "1"
                                },
                                "size": 774
                            },
                            {
                                "type": "leaf",
                                "data": {
                                    "views": [
                                        "vrdc"
                                    ],
                                    "activeView": "vrdc",
                                    "id": "2"
                                },
                                "size": 176
                            }
                        ],
                        "size": 1900
                    }
                ],
                "size": 950
            },
            "width": 1900,
            "height": 950,
            "orientation": Orientation.HORIZONTAL
        },
        "panels": {
            "gyro": {
                "id": "gyro",
                "contentComponent": "gyro",
                "title": "Gyroscope"
            },
            "vrdc": {
                "id": "vrdc",
                "contentComponent": "vrdc",
                "title": "VR Distance Configuration"
            }
        },
        "activeGroup": "2"
    })
}