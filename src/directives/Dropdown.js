
let lastClick = {}

function toggle (ev, node, className, on) {
  if (on === undefined || on === null) {
    on = !node.classList.contains(className)
  }
  if (on) {
    node.classList.add(className)
    lastClick = { node: node, target: ev.target, timeStamp: ev.timeStamp }
  } else {
    node.classList.remove(className)
    lastClick = {}
  }
}

function clickElem (ev, node) {
  toggle(ev, node, 'open')
}

function clickWindow (ev) {
  if (!lastClick.target ||
      (ev.target === lastClick.target && ev.timeStamp === lastClick.timeStamp)) {
    return
  }
  toggle(ev, lastClick.node, 'open', false)
}

function keyUp (ev) {
  if (lastClick.node && ev.keyCode === 27) {
    toggle(ev, lastClick.node, 'open', false)
  }
}

let click

export default {
  bind (el, binding) {
    click = (ev) => { clickElem(ev, el) }
    el.addEventListener('click', click)
    window.addEventListener('click', clickWindow)
    window.addEventListener('keyup', keyUp)
  },
  unbind (el, binding) {
    el.removeEventListener('click', click)
    window.removeEventListener('click', clickWindow)
    window.removeEventListener('keyup', keyUp)
  }
}
