release:
        @npm run check
        @npm run build
        @cargo build --release

build:
	@npm run build

compile:
	@cargo build

dev:
	@npm run dev

start:
	@npm start

dev:
	@num run dev

check:
	@npm run check
