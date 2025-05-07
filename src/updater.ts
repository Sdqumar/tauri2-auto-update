import {ask} from "@tauri-apps/plugin-dialog";
import {check} from "@tauri-apps/plugin-updater";
import { relaunch} from "@tauri-apps/plugin-process";

export async function checkForAppUpdates() {
    // Log that we are checking for updates
    console.log("Checking for updates...");

    try {
        const update = await check();

        if (update) {
            const yes = await ask(
                `
Update to ${update.version} is available!
Release notes: ${update.body}
                `,
                {
                    title: "Update Now!",
                    kind: "info",
                    okLabel: "Update",
                    cancelLabel: "Cancel",
                },
            );

            if (yes) {
                console.log("Downloading and installing update...");
                await update.downloadAndInstall();
                console.log("Update downloaded, relaunching application...");
                // Force relaunch instead of just exit
                await relaunch();
            }
        }
    } catch (error) {
        console.error("Update error:", error);
    }
}