<template>
    <section>
        <h1>Settings</h1>

        <div class="setting">
            <div class="setting-info">
                <h3 class="setting-title">Application data</h3>
                <p class="setting-description">
                    Export and import application data in case you need a
                    backup
                </p>
            </div>
            <div class="setting-actions">
                <button @click="handleExport">Export</button>
                <div>
                    <label for="fileInput">Import</label>
                    <input id="fileInput" type="file" multiple="false" accept="application/json" @change="handleImport"
                        name="Import" />
                </div>
            </div>
        </div>
    </section>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { writeTextFile, BaseDirectory } from "@tauri-apps/api/fs";
import { NoteStorage } from "../utils/interfaces";
import { store } from "../utils/store";

async function handleExport() {
    const data: NoteStorage = await invoke("get_all_notes");
    const stringifiedData = JSON.stringify(data);

    await writeTextFile("data.json", stringifiedData, {
        dir: BaseDirectory.Desktop,
    });
}

async function handleImport(event: Event) {
    if (event.target) {
        const files: FileList | null = (event.target as HTMLInputElement).files;
        if (files) {
            const file = files[0];
            if (file.type !== "application/json") return;
            const reader = new FileReader();
            reader.onload = async (event) => {
                if (event && event.target) {
                    const content = event.target.result;
                    if (content !== "") {
                        const importedNotes = JSON.parse(content as string);
                        const t = await invoke("import_notes", {
                            notes: importedNotes,
                        });
                        store.lastUpdate = Date.now();
                    }
                }
            };
            reader.onloadend = () => {
                const input: HTMLElement | null = document.getElementById("fileInput");
                if (input) {
                    (input as HTMLInputElement).value = "";
                }
            };
            reader.readAsText(file);
        }
    }
}
</script>

<style scoped>
section {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    padding: var(--padding-inner);
}

input[type="file"] {
    color: transparent;
}

.setting {
    display: flex;
    flex-direction: column;
    gap: var(--small-gap);
}

.setting-info {
    display: flex;
    flex-direction: column;
}

.setting-description {
    opacity: 0.7;
}

.setting-actions {
    display: flex;
    gap: var(--big-gap);
    align-items: center;
}
</style>
