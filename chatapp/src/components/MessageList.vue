<script setup lang="ts">
import { computed, nextTick, onMounted, onUpdated, watch } from 'vue';
import { useAuthStore } from '../stores/authStore';
import { Message } from '../types';

const authStore = useAuthStore()

// computed
const messages = computed(() => authStore.getMessagesForActiveChannel)
const activeChannelId = computed(() => authStore.activeChannel && authStore.activeChannel.id)

// watch
watch(
  messages,
  (_newValue, _oldValue) => {
    nextTick(() => {
      scrollToBottom()
    })
  },
  { deep: true }
)
watch(
  activeChannelId,
  (newChannelId, _oldValue) => {
    if (newChannelId) {
      fetchMessages(newChannelId)
    }
  }
)

// methods
const scrollToBottom = () => {

}
const fetchMessages = (channelId: number) => {
  authStore.fetchMessagesForChannel(channelId)
}
const getSender = (userId: number) => {
  return authStore.getUserById(userId)
}
const getMessageContent = (msg: Message) => {
  if (msg.senderId === authStore.user!.id) {
    return msg.content
  } else {
    return msg.modifiedContent && msg.modifiedContent.trim() !== '' ? msg.modifiedContent.trim(): msg.content
  }
}

// hook
onMounted(() => {
    activeChannelId.value && fetchMessages(activeChannelId.value!)
})

onUpdated(() => {
  scrollToBottom()
})
</script>

<template>
  <div class="flex-1 overflow-y-auto p-5 mb-10" ref="messageContainer">
    <div v-if="messages.length === 0" class="text-center text-gray-400 mt-5">
      No messages in this channel yet. {{ activeChannelId }}
    </div>
    <div v-else>
      <div v-for="message in messages" :key="message.id" class="flex items-start mb-5">
        <img :src="`https://ui-avatars.com/api/?name=${getSender(message.senderId)?.fullname.replace(' ', '+')}`" class="w-10 h-10 rounded-full mr-3" alt="Avatar" />
        <div class="max-w-4/5">
          <div class="flex items-center mb-1">
            <span class="font-bold mr-2">{{ getSender(message.senderId)?.fullname }}</span>
            <span class="text-xs text-gray-500">{{ message.formattedCreatedAt }}</span>
          </div>
          <div class="text-sm leading-relaxed break-words whitespace-pre-wrap">{{ getMessageContent(message) }}</div>
        </div>
      </div>
    </div>
  </div>
</template>
