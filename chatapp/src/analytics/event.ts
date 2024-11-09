import { AnalyticsEvent, AnalyticsEventSchema, AppExitEvent_ExitCode, EventContext } from "../gen/messages_pb";
import { create, toBinary } from "@bufbuild/protobuf";

const URL = 'http://localhost:6690/api/event'

export async function sendAppStartEvent(context: EventContext, token: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'appStart',
      value: {}
    }
  });
  await sendEvent(event, token)
}

export async function sendAppExitEvent(context: EventContext, token: string, exitCode: AppExitEvent_ExitCode) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'appExit',
      value: {
        exitCode
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendUserLoginEvent(context: EventContext, token: string, email: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'userLogin',
      value: {
        email
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendUserLogoutEvent(context: EventContext, token: string, email: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'userLogout',
      value: {
        email
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendUserRegisterEvent(context: EventContext, token: string, email: string, workspaceId: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'userRegister',
      value: {
        email,
        workspaceId
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendChatCreatedEvent(context: EventContext, token: string, workspaceId: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'chatCreated',
      value: {
        workspaceId
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendMessageSentEvent(context: EventContext, token: string, chatId: string, type: string, size: number, totalFiles: number) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'messageSent',
      value: {
        chatId,
        type,
        size,
        totalFiles
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendChatJoinedEvent(context: EventContext, token: string, chatId: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'chatJoined',
      value: {
        chatId,
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendChatLeftEvent(context: EventContext, token: string, chatId: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'chatLeft',
      value: {
        chatId,
      }
    }
  });
  await sendEvent(event, token)
}

export async function sendNavigationEvent(context: EventContext, token: string, from: string, to: string) {
  const event = create(AnalyticsEventSchema, {
    context,
    eventType: {
      case: 'navigation',
      value: {
        from,
        to
      }
    }
  });
  await sendEvent(event, token)
}

async function sendEvent(event: AnalyticsEvent, token: string) {
  console.log('event:', event);
  try {
    const data = toBinary(AnalyticsEventSchema, event);
    // attach token to the url
    let url = `${URL}?token=${token}`
    if (navigator.sendBeacon(url, data)) {
      console.log('sendBeacon');
    } else {
      await fetch(url, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/protobuf'
        },
        body: data
      });
    }
  } catch (error) {
    console.error('sendEvent error:', error)
  }
}
