
export function vlmjsthunk() {

}

export function vlmadds(v, l, m) { return v + l + m };

export function vlmusestack(vlmapp) {
    if (vlmapp !== VLMAPP['vlmapp-3']) {
        throw new Error("Passed wrong Variant");
    }
}

export class VlMApp {
    vlmapp() {}
  }

export const VLMAPP = {
    'vlmapp-1': 'vlmapp-1',
    'vlmapp-2': 'vlmapp-2',
    'vlmapp-3': 'vlmapp-3',
    'vlmapp-4': 'vlmapp-4',
}
