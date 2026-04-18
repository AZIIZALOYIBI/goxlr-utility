#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <errno.h>

// =============================================
// GoXLR - SINGING Mic Profile Binary Installer
// =============================================

static const char PROFILE_DATA[] =
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
    "<MicProfileTree>\n"
    "  <dspTreeMicProfile"
    " MIC_EQ_31.5HZ_GAIN=\"-1\" MIC_EQ_63HZ_GAIN=\"-4\" MIC_EQ_125HZ_GAIN=\"-3\""
    " MIC_EQ_250HZ_GAIN=\"2\" MIC_EQ_500HZ_GAIN=\"3\" MIC_EQ_1KHZ_GAIN=\"5\""
    " MIC_EQ_2KHZ_GAIN=\"6\" MIC_EQ_4KHZ_GAIN=\"5\" MIC_EQ_8KHZ_GAIN=\"3\""
    " MIC_EQ_16KHZ_GAIN=\"2\" MIC_MINI_EQ_90HZ_GAIN=\"-3\" MIC_MINI_EQ_250HZ_GAIN=\"2\""
    " MIC_MINI_EQ_500HZ_GAIN=\"3\" MIC_MINI_EQ_1KHZ_GAIN=\"5\" MIC_MINI_EQ_3KHZ_GAIN=\"5\""
    " MIC_MINI_EQ_8KHZ_GAIN=\"3\""
    " MIC_EQ_31.5HZ_F=\"31.50000000000000000000\""
    " MIC_EQ_63HZ_F=\"63.00000000000000000000\""
    " MIC_EQ_125HZ_F=\"125.00000000000000000000\""
    " MIC_EQ_250HZ_F=\"250.00000000000000000000\""
    " MIC_EQ_500HZ_F=\"500.00000000000000000000\""
    " MIC_EQ_1KHZ_F=\"1000.00000000000000000000\""
    " MIC_EQ_2KHZ_F=\"2000.00000000000000000000\""
    " MIC_EQ_4KHZ_F=\"4000.00000000000000000000\""
    " MIC_EQ_8KHZ_F=\"8000.00000000000000000000\""
    " MIC_EQ_16KHZ_F=\"16000.00000000000000000000\""
    " MIC_MINI_EQ_90HZ_F=\"90\" MIC_MINI_EQ_250HZ_F=\"250\""
    " MIC_MINI_EQ_500HZ_F=\"500\" MIC_MINI_EQ_1KHZ_F=\"1000\""
    " MIC_MINI_EQ_3KHZ_F=\"3000\" MIC_MINI_EQ_8KHZ_F=\"8000\""
    " MIC_DEESS_AMOUNT=\"45\""
    " MIC_COMP_THRESHOLD=\"-22\" MIC_COMP_RATIO=\"3\""
    " MIC_COMP_ATTACK=\"5\" MIC_COMP_RELEASE=\"20\" MIC_COMP_MAKEUPGAIN=\"8\""
    " MIC_GATE_MACRO_AMOUNT=\"0\" MIC_GATE_THRESOLD=\"-52\""
    " MIC_GATE_ATTACK=\"1\" MIC_GATE_RELEASE=\"35\""
    " MIC_GATE_ENABLE=\"1\" MIC_GATE_ATTEN=\"100\""
    " MIC_COMP_SELECT=\"1\" BLEEP_LEVEL=\"-10\" MIC_GATE_MODE=\"2\"/>
    "
    "  <setupTreeMicProfile MIC_TYPE=\"0\""
    " DYNAMIC_MIC_GAIN=\"1966080\""
    " CONDENSER_MIC_GAIN=\"1966080\""
    " TRS_MIC_GAIN=\"1966080\"/>
    "
    "  <micProfileUIMicProfile eqAdvanced=\"0\" compAdvanced=\"0\""
    " gateAdvanced=\"0\" eqFineTuneEnabled=\"0\"/>
    "
    "</MicProfileTree>\n";

static const char *PROFILE_NAME = \"SINGING.goxlrMicProfile\";

void create_dir(const char *path) {
    char tmp[512];
    char *p = NULL;
    size_t len;
    snprintf(tmp, sizeof(tmp), "%s", path);
    len = strlen(tmp);
    if (tmp[len - 1] == '/') tmp[len - 1] = 0;
    for (p = tmp + 1; *p; p++) {
        if (*p == '/') {
            *p = 0;
            mkdir(tmp, 0755);
            *p = '/';
        }
    }
    mkdir(tmp, 0755);
}

int main(void) {
    char profile_dir[512];
    char profile_path[512];
    const char *home = getenv("HOME");
    FILE *f;

    printf("🎙️  GoXLR SINGING Mic Profile - Binary Installer\n");
    printf("=================================================\n\n");

    if (!home) {
        fprintf(stderr, "❌ Cannot find HOME directory!\n");
        return 1;
    }

    snprintf(profile_dir, sizeof(profile_dir),
             "%s/.config/goxlr-utility/mic-profiles", home);
    snprintf(profile_path, sizeof(profile_path),
             "%s/%s", profile_dir, PROFILE_NAME);

    printf("📁 Creating directory: %s\n", profile_dir);
    create_dir(profile_dir);

    f = fopen(profile_path, "w");
    if (!f) {
        fprintf(stderr, "❌ Cannot write to: %s\nError: %s\n",
                profile_path, strerror(errno));
        return 1;
    }

    fwrite(PROFILE_DATA, 1, strlen(PROFILE_DATA), f);
    fclose(f);

    printf("✅ Profile installed to:\n   %s\n\n", profile_path);
    printf("🎉 Done! Open GoXLR Utility and select 'SINGING' from Mic Profiles.\n");

    return 0;
}