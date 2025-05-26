cd $(dirname $0)
npx tsc -d ./index.ts -t esnext -m esnext --skipLibCheck --lib esnext,dom