export class PubSub {
    #subscribers = []

    subscribe(name,func) {
        this.#subscribers.push({
            name,
            func
        })
    }

    unsubscribe(name) {
        this.#subscribers = this.#subscribers.filter((sub)=> sub!== name)
    }

    emit(event) {
        this.#subscribers.forEach(({func})=> func(event))
    }
}